#include "ThumbnailWidget.h"
#include <QPixmap>

ThumbnailWidget::ThumbnailWidget(QWidget *parent)
    : QListWidget(parent)
    , m_document(nullptr)
{
    setViewMode(QListView::IconMode);
    setIconSize(QSize(150, 200));
    setSpacing(10);
    setMovement(QListView::Static);
    setResizeMode(QListView::Adjust);
    setUniformItemSizes(true);
    
    // Apply dark theme styling
    setStyleSheet(R"(
        QListWidget {
            background-color: #2b2b2b;
            border: none;
            outline: none;
        }
        QListWidget::item {
            background-color: #3c3c3c;
            border: 2px solid #4a4a4a;
            border-radius: 4px;
            padding: 8px;
            color: #ffffff;
        }
        QListWidget::item:hover {
            background-color: #4a4a4a;
            border-color: #0d7377;
        }
        QListWidget::item:selected {
            background-color: #4a4a4a;
            border-color: #14ffec;
        }
    )");
    
    connect(this, &QListWidget::itemClicked, this, &ThumbnailWidget::onItemClicked);
}

ThumbnailWidget::~ThumbnailWidget()
{
}

void ThumbnailWidget::setDocument(Poppler::Document *document)
{
    clear();
    m_document = document;
    
    if (m_document) {
        generateThumbnails();
    }
}

void ThumbnailWidget::clear()
{
    QListWidget::clear();
    m_document = nullptr;
}

void ThumbnailWidget::generateThumbnails()
{
    if (!m_document) {
        return;
    }
    
    int numPages = m_document->numPages();
    
    for (int i = 0; i < numPages; ++i) {
        Poppler::Page *page = m_document->page(i);
        if (!page) {
            continue;
        }
        
        // Render thumbnail at lower DPI for performance
        QImage image = page->renderToImage(72, 72);
        
        if (!image.isNull()) {
            QPixmap pixmap = QPixmap::fromImage(image);
            
            // Scale to thumbnail size while maintaining aspect ratio
            pixmap = pixmap.scaled(150, 200, Qt::KeepAspectRatio, Qt::SmoothTransformation);
            
            QListWidgetItem *item = new QListWidgetItem(this);
            item->setIcon(QIcon(pixmap));
            item->setText(QString("Page %1").arg(i + 1));
            item->setData(Qt::UserRole, i);
            item->setTextAlignment(Qt::AlignCenter);
        }
        
        delete page;
    }
}

void ThumbnailWidget::onItemClicked(QListWidgetItem *item)
{
    if (item) {
        int pageNumber = item->data(Qt::UserRole).toInt();
        emit pageSelected(pageNumber);
    }
}
