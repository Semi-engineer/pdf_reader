#include "PDFViewer.h"
#include <QWheelEvent>
#include <QPainter>
#include <QPrinter>
#include <QDebug>

PDFViewer::PDFViewer(QWidget *parent)
    : QScrollArea(parent)
    , m_document(nullptr)
    , m_imageLabel(new QLabel)
    , m_currentPage(0)
    , m_zoomLevel(100.0)
    , m_rotation(0)
    , m_fitToWindowMode(false)
{
    m_imageLabel->setAlignment(Qt::AlignCenter);
    m_imageLabel->setStyleSheet("background-color: #323232; padding: 20px;");
    setWidget(m_imageLabel);
    setAlignment(Qt::AlignCenter);
    
    // Set dark background
    setStyleSheet("QScrollArea { background-color: #2b2b2b; }");
}

PDFViewer::~PDFViewer()
{
    closeDocument();
}

bool PDFViewer::loadDocument(const QString &filePath)
{
    closeDocument();
    
    m_document = Poppler::Document::load(filePath);
    
    if (!m_document || m_document->isLocked()) {
        delete m_document;
        m_document = nullptr;
        return false;
    }
    
    // Set render hints for better quality
    m_document->setRenderHint(Poppler::Document::Antialiasing);
    m_document->setRenderHint(Poppler::Document::TextAntialiasing);
    m_document->setRenderHint(Poppler::Document::TextHinting);
    
    m_filePath = filePath;
    m_currentPage = 0;
    m_rotation = 0;
    
    renderPage();
    emit pageChanged(m_currentPage, pageCount());
    
    return true;
}

void PDFViewer::closeDocument()
{
    if (m_document) {
        delete m_document;
        m_document = nullptr;
    }
    m_imageLabel->clear();
    m_currentPage = 0;
}

bool PDFViewer::hasDocument() const
{
    return m_document != nullptr;
}

int PDFViewer::pageCount() const
{
    return m_document ? m_document->numPages() : 0;
}

void PDFViewer::setCurrentPage(int page)
{
    if (!m_document || page < 0 || page >= pageCount()) {
        return;
    }
    
    m_currentPage = page;
    renderPage();
    emit pageChanged(m_currentPage, pageCount());
}

void PDFViewer::nextPage()
{
    if (m_currentPage < pageCount() - 1) {
        setCurrentPage(m_currentPage + 1);
    }
}

void PDFViewer::previousPage()
{
    if (m_currentPage > 0) {
        setCurrentPage(m_currentPage - 1);
    }
}

void PDFViewer::firstPage()
{
    setCurrentPage(0);
}

void PDFViewer::lastPage()
{
    setCurrentPage(pageCount() - 1);
}

void PDFViewer::setZoom(qreal zoom)
{
    if (!m_document) {
        return;
    }
    
    m_fitToWindowMode = false;
    m_zoomLevel = qBound(25.0, zoom, 400.0);
    renderPage();
    emit zoomChanged(m_zoomLevel);
}

void PDFViewer::zoomIn()
{
    setZoom(m_zoomLevel + 25);
}

void PDFViewer::zoomOut()
{
    setZoom(m_zoomLevel - 25);
}

void PDFViewer::fitToWindow()
{
    if (!m_document) {
        return;
    }
    
    m_fitToWindowMode = true;
    renderPage();
}

void PDFViewer::rotateLeft()
{
    m_rotation = (m_rotation - 90) % 360;
    if (m_rotation < 0) m_rotation += 360;
    renderPage();
}

void PDFViewer::rotateRight()
{
    m_rotation = (m_rotation + 90) % 360;
    renderPage();
}

void PDFViewer::renderPage()
{
    if (!m_document) {
        return;
    }
    
    Poppler::Page *page = m_document->page(m_currentPage);
    if (!page) {
        return;
    }
    
    QSizeF pageSize = page->pageSizeF();
    qreal dpi = 72.0;
    
    if (m_fitToWindowMode) {
        // Calculate zoom to fit window
        QSize viewportSize = viewport()->size();
        qreal widthRatio = (viewportSize.width() - 40) / (pageSize.width() / 72.0);
        qreal heightRatio = (viewportSize.height() - 40) / (pageSize.height() / 72.0);
        dpi = qMin(widthRatio, heightRatio) * 72.0;
        m_zoomLevel = (dpi / 72.0) * 100.0;
        emit zoomChanged(m_zoomLevel);
    } else {
        dpi = 72.0 * (m_zoomLevel / 100.0);
    }
    
    // Render the page
    QImage image = page->renderToImage(dpi, dpi, -1, -1, -1, -1, 
                                       static_cast<Poppler::Page::Rotation>(m_rotation / 90));
    
    if (!image.isNull()) {
        m_imageLabel->setPixmap(QPixmap::fromImage(image));
    }
    
    delete page;
}

void PDFViewer::resizeEvent(QResizeEvent *event)
{
    QScrollArea::resizeEvent(event);
    
    if (m_fitToWindowMode && m_document) {
        renderPage();
    }
}

void PDFViewer::wheelEvent(QWheelEvent *event)
{
    if (event->modifiers() & Qt::ControlModifier) {
        // Zoom with Ctrl + Mouse Wheel
        if (event->angleDelta().y() > 0) {
            zoomIn();
        } else {
            zoomOut();
        }
        event->accept();
    } else {
        QScrollArea::wheelEvent(event);
    }
}

void PDFViewer::print(QPrinter *printer)
{
    if (!m_document || !printer) {
        return;
    }
    
    QPainter painter;
    painter.begin(printer);
    
    for (int i = 0; i < pageCount(); ++i) {
        if (i > 0) {
            printer->newPage();
        }
        
        Poppler::Page *page = m_document->page(i);
        if (!page) {
            continue;
        }
        
        QImage image = page->renderToImage(300, 300);
        
        QRect rect = painter.viewport();
        QSize size = image.size();
        size.scale(rect.size(), Qt::KeepAspectRatio);
        
        painter.setViewport(rect.x(), rect.y(), size.width(), size.height());
        painter.setWindow(image.rect());
        painter.drawImage(0, 0, image);
        
        delete page;
    }
    
    painter.end();
}

QList<QRectF> PDFViewer::search(const QString &text, int page)
{
    QList<QRectF> results;
    
    if (!m_document || page < 0 || page >= pageCount()) {
        return results;
    }
    
    Poppler::Page *pdfPage = m_document->page(page);
    if (!pdfPage) {
        return results;
    }
    
    results = pdfPage->search(text, Poppler::Page::CaseInsensitive);
    
    delete pdfPage;
    return results;
}
