#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>
#include <QToolBar>
#include <QStatusBar>
#include <QLabel>
#include <QSlider>
#include <QSpinBox>
#include <QDockWidget>

class PDFViewer;
class ThumbnailWidget;
class SearchDialog;

class MainWindow : public QMainWindow
{
    Q_OBJECT

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow();
    
    void openFile(const QString &filePath);

private slots:
    void onOpenFile();
    void onPrint();
    void onZoomIn();
    void onZoomOut();
    void onZoomFit();
    void onZoomActual();
    void onZoomChanged(int value);
    void onPreviousPage();
    void onNextPage();
    void onFirstPage();
    void onLastPage();
    void onGoToPage();
    void onRotateLeft();
    void onRotateRight();
    void onSearch();
    void onToggleFullScreen();
    void onAbout();
    void updatePageInfo(int currentPage, int totalPages);
    void updateZoomInfo(qreal zoom);

private:
    void setupUI();
    void createActions();
    void createToolBar();
    void createStatusBar();
    void createDockWidgets();
    void updateActions();
    
    PDFViewer *m_pdfViewer;
    ThumbnailWidget *m_thumbnailWidget;
    SearchDialog *m_searchDialog;
    
    // Toolbar widgets
    QToolBar *m_toolBar;
    QSlider *m_zoomSlider;
    QSpinBox *m_pageSpinBox;
    QLabel *m_pageLabel;
    QLabel *m_zoomLabel;
    
    // Actions
    QAction *m_openAction;
    QAction *m_printAction;
    QAction *m_exitAction;
    QAction *m_zoomInAction;
    QAction *m_zoomOutAction;
    QAction *m_zoomFitAction;
    QAction *m_zoomActualAction;
    QAction *m_previousPageAction;
    QAction *m_nextPageAction;
    QAction *m_firstPageAction;
    QAction *m_lastPageAction;
    QAction *m_rotateLeftAction;
    QAction *m_rotateRightAction;
    QAction *m_searchAction;
    QAction *m_fullScreenAction;
    QAction *m_aboutAction;
    
    // Dock widgets
    QDockWidget *m_thumbnailDock;
};

#endif // MAINWINDOW_H
