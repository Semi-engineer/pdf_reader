#include "MainWindow.h"
#include "PDFViewer.h"
#include "ThumbnailWidget.h"
#include "SearchDialog.h"

#include <QFileDialog>
#include <QMessageBox>
#include <QMenuBar>
#include <QMenu>
#include <QToolButton>
#include <QHBoxLayout>
#include <QVBoxLayout>
#include <QStyle>
#include <QPrintDialog>
#include <QPrinter>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , m_pdfViewer(nullptr)
    , m_thumbnailWidget(nullptr)
    , m_searchDialog(nullptr)
    , m_toolBar(nullptr)
    , m_zoomSlider(nullptr)
    , m_pageSpinBox(nullptr)
    , m_pageLabel(nullptr)
    , m_zoomLabel(nullptr)
{
    setupUI();
    createActions();
    createToolBar();
    createStatusBar();
    createDockWidgets();
    updateActions();
    
    setWindowTitle("PDF Reader");
    resize(1200, 800);
    
    // Apply modern stylesheet
    setStyleSheet(R"(
        QMainWindow {
            background-color: #2b2b2b;
        }
        QToolBar {
            background-color: #3c3c3c;
            border: none;
            spacing: 8px;
            padding: 4px;
        }
        QToolButton {
            background-color: transparent;
            border: none;
            border-radius: 4px;
            padding: 6px;
            color: #ffffff;
        }
        QToolButton:hover {
            background-color: #4a4a4a;
        }
        QToolButton:pressed {
            background-color: #5a5a5a;
        }
        QStatusBar {
            background-color: #3c3c3c;
            color: #ffffff;
        }
        QDockWidget {
            background-color: #2b2b2b;
            color: #ffffff;
            titlebar-close-icon: url(close.png);
            titlebar-normal-icon: url(float.png);
        }
        QDockWidget::title {
            background-color: #3c3c3c;
            padding: 6px;
        }
        QSlider::groove:horizontal {
            border: 1px solid #5a5a5a;
            height: 6px;
            background: #4a4a4a;
            border-radius: 3px;
        }
        QSlider::handle:horizontal {
            background: #0d7377;
            border: 1px solid #0d7377;
            width: 14px;
            margin: -5px 0;
            border-radius: 7px;
        }
        QSlider::handle:horizontal:hover {
            background: #14ffec;
        }
        QSpinBox {
            background-color: #4a4a4a;
            border: 1px solid #5a5a5a;
            border-radius: 4px;
            padding: 4px;
            color: #ffffff;
            min-width: 60px;
        }
        QSpinBox::up-button, QSpinBox::down-button {
            background-color: #5a5a5a;
            border: none;
            width: 16px;
        }
        QSpinBox::up-button:hover, QSpinBox::down-button:hover {
            background-color: #6a6a6a;
        }
        QLabel {
            color: #ffffff;
        }
    )");
}

MainWindow::~MainWindow()
{
}

void MainWindow::setupUI()
{
    m_pdfViewer = new PDFViewer(this);
    setCentralWidget(m_pdfViewer);
    
    connect(m_pdfViewer, &PDFViewer::pageChanged, this, &MainWindow::updatePageInfo);
    connect(m_pdfViewer, &PDFViewer::zoomChanged, this, &MainWindow::updateZoomInfo);
}

void MainWindow::createActions()
{
    // File menu actions
    m_openAction = new QAction(style()->standardIcon(QStyle::SP_DialogOpenButton), tr("&Open..."), this);
    m_openAction->setShortcut(QKeySequence::Open);
    m_openAction->setStatusTip(tr("Open a PDF file"));
    connect(m_openAction, &QAction::triggered, this, &MainWindow::onOpenFile);
    
    m_printAction = new QAction(style()->standardIcon(QStyle::SP_FileDialogDetailedView), tr("&Print..."), this);
    m_printAction->setShortcut(QKeySequence::Print);
    m_printAction->setStatusTip(tr("Print the document"));
    m_printAction->setEnabled(false);
    connect(m_printAction, &QAction::triggered, this, &MainWindow::onPrint);
    
    m_exitAction = new QAction(tr("E&xit"), this);
    m_exitAction->setShortcut(QKeySequence::Quit);
    m_exitAction->setStatusTip(tr("Exit the application"));
    connect(m_exitAction, &QAction::triggered, this, &QWidget::close);
    
    // View menu actions
    m_zoomInAction = new QAction(style()->standardIcon(QStyle::SP_ArrowUp), tr("Zoom &In"), this);
    m_zoomInAction->setShortcut(QKeySequence::ZoomIn);
    m_zoomInAction->setStatusTip(tr("Zoom in"));
    m_zoomInAction->setEnabled(false);
    connect(m_zoomInAction, &QAction::triggered, this, &MainWindow::onZoomIn);
    
    m_zoomOutAction = new QAction(style()->standardIcon(QStyle::SP_ArrowDown), tr("Zoom &Out"), this);
    m_zoomOutAction->setShortcut(QKeySequence::ZoomOut);
    m_zoomOutAction->setStatusTip(tr("Zoom out"));
    m_zoomOutAction->setEnabled(false);
    connect(m_zoomOutAction, &QAction::triggered, this, &MainWindow::onZoomOut);
    
    m_zoomFitAction = new QAction(tr("&Fit to Window"), this);
    m_zoomFitAction->setStatusTip(tr("Fit page to window"));
    m_zoomFitAction->setEnabled(false);
    connect(m_zoomFitAction, &QAction::triggered, this, &MainWindow::onZoomFit);
    
    m_zoomActualAction = new QAction(tr("&Actual Size"), this);
    m_zoomActualAction->setShortcut(tr("Ctrl+0"));
    m_zoomActualAction->setStatusTip(tr("Actual size"));
    m_zoomActualAction->setEnabled(false);
    connect(m_zoomActualAction, &QAction::triggered, this, &MainWindow::onZoomActual);
    
    m_fullScreenAction = new QAction(tr("&Full Screen"), this);
    m_fullScreenAction->setShortcut(Qt::Key_F11);
    m_fullScreenAction->setStatusTip(tr("Toggle full screen mode"));
    m_fullScreenAction->setCheckable(true);
    connect(m_fullScreenAction, &QAction::triggered, this, &MainWindow::onToggleFullScreen);
    
    // Navigation actions
    m_previousPageAction = new QAction(style()->standardIcon(QStyle::SP_ArrowLeft), tr("&Previous Page"), this);
    m_previousPageAction->setShortcut(QKeySequence::MoveToPreviousPage);
    m_previousPageAction->setStatusTip(tr("Go to previous page"));
    m_previousPageAction->setEnabled(false);
    connect(m_previousPageAction, &QAction::triggered, this, &MainWindow::onPreviousPage);
    
    m_nextPageAction = new QAction(style()->standardIcon(QStyle::SP_ArrowRight), tr("&Next Page"), this);
    m_nextPageAction->setShortcut(QKeySequence::MoveToNextPage);
    m_nextPageAction->setStatusTip(tr("Go to next page"));
    m_nextPageAction->setEnabled(false);
    connect(m_nextPageAction, &QAction::triggered, this, &MainWindow::onNextPage);
    
    m_firstPageAction = new QAction(tr("&First Page"), this);
    m_firstPageAction->setShortcut(QKeySequence::MoveToStartOfDocument);
    m_firstPageAction->setStatusTip(tr("Go to first page"));
    m_firstPageAction->setEnabled(false);
    connect(m_firstPageAction, &QAction::triggered, this, &MainWindow::onFirstPage);
    
    m_lastPageAction = new QAction(tr("&Last Page"), this);
    m_lastPageAction->setShortcut(QKeySequence::MoveToEndOfDocument);
    m_lastPageAction->setStatusTip(tr("Go to last page"));
    m_lastPageAction->setEnabled(false);
    connect(m_lastPageAction, &QAction::triggered, this, &MainWindow::onLastPage);
    
    m_rotateLeftAction = new QAction(tr("Rotate &Left"), this);
    m_rotateLeftAction->setShortcut(tr("Ctrl+L"));
    m_rotateLeftAction->setStatusTip(tr("Rotate page left"));
    m_rotateLeftAction->setEnabled(false);
    connect(m_rotateLeftAction, &QAction::triggered, this, &MainWindow::onRotateLeft);
    
    m_rotateRightAction = new QAction(tr("Rotate &Right"), this);
    m_rotateRightAction->setShortcut(tr("Ctrl+R"));
    m_rotateRightAction->setStatusTip(tr("Rotate page right"));
    m_rotateRightAction->setEnabled(false);
    connect(m_rotateRightAction, &QAction::triggered, this, &MainWindow::onRotateRight);
    
    // Search action
    m_searchAction = new QAction(style()->standardIcon(QStyle::SP_FileDialogContentsView), tr("&Search..."), this);
    m_searchAction->setShortcut(QKeySequence::Find);
    m_searchAction->setStatusTip(tr("Search in document"));
    m_searchAction->setEnabled(false);
    connect(m_searchAction, &QAction::triggered, this, &MainWindow::onSearch);
    
    // Help menu actions
    m_aboutAction = new QAction(tr("&About"), this);
    m_aboutAction->setStatusTip(tr("About PDF Reader"));
    connect(m_aboutAction, &QAction::triggered, this, &MainWindow::onAbout);
    
    // Create menu bar
    QMenu *fileMenu = menuBar()->addMenu(tr("&File"));
    fileMenu->addAction(m_openAction);
    fileMenu->addAction(m_printAction);
    fileMenu->addSeparator();
    fileMenu->addAction(m_exitAction);
    
    QMenu *viewMenu = menuBar()->addMenu(tr("&View"));
    viewMenu->addAction(m_zoomInAction);
    viewMenu->addAction(m_zoomOutAction);
    viewMenu->addAction(m_zoomFitAction);
    viewMenu->addAction(m_zoomActualAction);
    viewMenu->addSeparator();
    viewMenu->addAction(m_fullScreenAction);
    
    QMenu *navMenu = menuBar()->addMenu(tr("&Navigate"));
    navMenu->addAction(m_firstPageAction);
    navMenu->addAction(m_previousPageAction);
    navMenu->addAction(m_nextPageAction);
    navMenu->addAction(m_lastPageAction);
    navMenu->addSeparator();
    navMenu->addAction(m_rotateLeftAction);
    navMenu->addAction(m_rotateRightAction);
    
    QMenu *editMenu = menuBar()->addMenu(tr("&Edit"));
    editMenu->addAction(m_searchAction);
    
    QMenu *helpMenu = menuBar()->addMenu(tr("&Help"));
    helpMenu->addAction(m_aboutAction);
}

void MainWindow::createToolBar()
{
    m_toolBar = addToolBar(tr("Main Toolbar"));
    m_toolBar->setMovable(false);
    m_toolBar->setIconSize(QSize(24, 24));
    
    // File operations
    m_toolBar->addAction(m_openAction);
    m_toolBar->addAction(m_printAction);
    m_toolBar->addSeparator();
    
    // Navigation
    m_toolBar->addAction(m_previousPageAction);
    m_toolBar->addAction(m_nextPageAction);
    
    // Page number
    m_pageSpinBox = new QSpinBox(this);
    m_pageSpinBox->setMinimum(1);
    m_pageSpinBox->setMaximum(1);
    m_pageSpinBox->setEnabled(false);
    connect(m_pageSpinBox, QOverload<int>::of(&QSpinBox::valueChanged), this, &MainWindow::onGoToPage);
    m_toolBar->addWidget(m_pageSpinBox);
    
    m_pageLabel = new QLabel(tr("/ 1"), this);
    m_toolBar->addWidget(m_pageLabel);
    m_toolBar->addSeparator();
    
    // Zoom controls
    m_toolBar->addAction(m_zoomOutAction);
    
    m_zoomSlider = new QSlider(Qt::Horizontal, this);
    m_zoomSlider->setMinimum(25);
    m_zoomSlider->setMaximum(400);
    m_zoomSlider->setValue(100);
    m_zoomSlider->setFixedWidth(150);
    m_zoomSlider->setEnabled(false);
    connect(m_zoomSlider, &QSlider::valueChanged, this, &MainWindow::onZoomChanged);
    m_toolBar->addWidget(m_zoomSlider);
    
    m_toolBar->addAction(m_zoomInAction);
    
    m_zoomLabel = new QLabel(tr("100%"), this);
    m_zoomLabel->setMinimumWidth(50);
    m_toolBar->addWidget(m_zoomLabel);
    m_toolBar->addSeparator();
    
    // Rotation
    m_toolBar->addAction(m_rotateLeftAction);
    m_toolBar->addAction(m_rotateRightAction);
    m_toolBar->addSeparator();
    
    // Search
    m_toolBar->addAction(m_searchAction);
}

void MainWindow::createStatusBar()
{
    statusBar()->showMessage(tr("Ready"));
}

void MainWindow::createDockWidgets()
{
    // Thumbnail dock
    m_thumbnailDock = new QDockWidget(tr("Pages"), this);
    m_thumbnailWidget = new ThumbnailWidget(this);
    m_thumbnailDock->setWidget(m_thumbnailWidget);
    m_thumbnailDock->setAllowedAreas(Qt::LeftDockWidgetArea | Qt::RightDockWidgetArea);
    addDockWidget(Qt::LeftDockWidgetArea, m_thumbnailDock);
    m_thumbnailDock->hide();
    
    connect(m_thumbnailWidget, &ThumbnailWidget::pageSelected, m_pdfViewer, &PDFViewer::setCurrentPage);
}

void MainWindow::updateActions()
{
    bool hasDocument = m_pdfViewer && m_pdfViewer->hasDocument();
    
    m_printAction->setEnabled(hasDocument);
    m_zoomInAction->setEnabled(hasDocument);
    m_zoomOutAction->setEnabled(hasDocument);
    m_zoomFitAction->setEnabled(hasDocument);
    m_zoomActualAction->setEnabled(hasDocument);
    m_previousPageAction->setEnabled(hasDocument);
    m_nextPageAction->setEnabled(hasDocument);
    m_firstPageAction->setEnabled(hasDocument);
    m_lastPageAction->setEnabled(hasDocument);
    m_rotateLeftAction->setEnabled(hasDocument);
    m_rotateRightAction->setEnabled(hasDocument);
    m_searchAction->setEnabled(hasDocument);
    m_zoomSlider->setEnabled(hasDocument);
    m_pageSpinBox->setEnabled(hasDocument);
}

void MainWindow::openFile(const QString &filePath)
{
    if (m_pdfViewer->loadDocument(filePath)) {
        setWindowTitle(tr("PDF Reader - %1").arg(QFileInfo(filePath).fileName()));
        statusBar()->showMessage(tr("Loaded: %1").arg(filePath), 3000);
        
        // Update thumbnail widget
        m_thumbnailWidget->setDocument(m_pdfViewer->document());
        m_thumbnailDock->show();
        
        updateActions();
        
        // Update page controls
        int totalPages = m_pdfViewer->pageCount();
        m_pageSpinBox->setMaximum(totalPages);
        m_pageSpinBox->setValue(1);
        m_pageLabel->setText(tr("/ %1").arg(totalPages));
    } else {
        QMessageBox::critical(this, tr("Error"), tr("Failed to open PDF file:\n%1").arg(filePath));
    }
}

void MainWindow::onOpenFile()
{
    QString filePath = QFileDialog::getOpenFileName(this, tr("Open PDF File"), QString(),
                                                     tr("PDF Files (*.pdf);;All Files (*)"));
    if (!filePath.isEmpty()) {
        openFile(filePath);
    }
}

void MainWindow::onPrint()
{
    if (!m_pdfViewer->hasDocument()) {
        return;
    }
    
    QPrinter printer;
    QPrintDialog dialog(&printer, this);
    dialog.setWindowTitle(tr("Print Document"));
    
    if (dialog.exec() == QDialog::Accepted) {
        m_pdfViewer->print(&printer);
    }
}

void MainWindow::onZoomIn()
{
    m_pdfViewer->zoomIn();
}

void MainWindow::onZoomOut()
{
    m_pdfViewer->zoomOut();
}

void MainWindow::onZoomFit()
{
    m_pdfViewer->fitToWindow();
}

void MainWindow::onZoomActual()
{
    m_pdfViewer->setZoom(100);
}

void MainWindow::onZoomChanged(int value)
{
    m_pdfViewer->setZoom(value);
}

void MainWindow::onPreviousPage()
{
    m_pdfViewer->previousPage();
}

void MainWindow::onNextPage()
{
    m_pdfViewer->nextPage();
}

void MainWindow::onFirstPage()
{
    m_pdfViewer->firstPage();
}

void MainWindow::onLastPage()
{
    m_pdfViewer->lastPage();
}

void MainWindow::onGoToPage()
{
    m_pdfViewer->setCurrentPage(m_pageSpinBox->value() - 1);
}

void MainWindow::onRotateLeft()
{
    m_pdfViewer->rotateLeft();
}

void MainWindow::onRotateRight()
{
    m_pdfViewer->rotateRight();
}

void MainWindow::onSearch()
{
    if (!m_searchDialog) {
        m_searchDialog = new SearchDialog(m_pdfViewer, this);
    }
    m_searchDialog->show();
    m_searchDialog->raise();
    m_searchDialog->activateWindow();
}

void MainWindow::onToggleFullScreen()
{
    if (isFullScreen()) {
        showNormal();
    } else {
        showFullScreen();
    }
}

void MainWindow::onAbout()
{
    QMessageBox::about(this, tr("About PDF Reader"),
                      tr("<h2>PDF Reader 1.0</h2>"
                         "<p>A modern PDF viewer built with Qt and Poppler.</p>"
                         "<p>Features:</p>"
                         "<ul>"
                         "<li>View PDF documents</li>"
                         "<li>Zoom and rotate pages</li>"
                         "<li>Navigate between pages</li>"
                         "<li>Search text in documents</li>"
                         "<li>Print documents</li>"
                         "<li>Thumbnail view</li>"
                         "</ul>"
                         "<p>© 2025 PDF Reader</p>"));
}

void MainWindow::updatePageInfo(int currentPage, int totalPages)
{
    m_pageSpinBox->blockSignals(true);
    m_pageSpinBox->setValue(currentPage + 1);
    m_pageSpinBox->blockSignals(false);
    
    m_pageLabel->setText(tr("/ %1").arg(totalPages));
    statusBar()->showMessage(tr("Page %1 of %2").arg(currentPage + 1).arg(totalPages));
}

void MainWindow::updateZoomInfo(qreal zoom)
{
    m_zoomSlider->blockSignals(true);
    m_zoomSlider->setValue(static_cast<int>(zoom));
    m_zoomSlider->blockSignals(false);
    
    m_zoomLabel->setText(tr("%1%").arg(static_cast<int>(zoom)));
}
