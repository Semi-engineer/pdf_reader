#ifndef PDFVIEWER_H
#define PDFVIEWER_H

#include <QScrollArea>
#include <QLabel>
#include <poppler-qt6.h>

class PDFViewer : public QScrollArea
{
    Q_OBJECT

public:
    explicit PDFViewer(QWidget *parent = nullptr);
    ~PDFViewer();
    
    bool loadDocument(const QString &filePath);
    void closeDocument();
    bool hasDocument() const;
    
    Poppler::Document* document() const { return m_document; }
    int pageCount() const;
    int currentPage() const { return m_currentPage; }
    qreal zoom() const { return m_zoomLevel; }
    
    void print(QPrinter *printer);

public slots:
    void setCurrentPage(int page);
    void nextPage();
    void previousPage();
    void firstPage();
    void lastPage();
    
    void setZoom(qreal zoom);
    void zoomIn();
    void zoomOut();
    void fitToWindow();
    
    void rotateLeft();
    void rotateRight();
    
    QList<QRectF> search(const QString &text, int page);

signals:
    void pageChanged(int currentPage, int totalPages);
    void zoomChanged(qreal zoom);

protected:
    void resizeEvent(QResizeEvent *event) override;
    void wheelEvent(QWheelEvent *event) override;

private:
    void renderPage();
    void updatePageDisplay();
    
    Poppler::Document *m_document;
    QLabel *m_imageLabel;
    
    int m_currentPage;
    qreal m_zoomLevel;
    int m_rotation;
    
    QString m_filePath;
    bool m_fitToWindowMode;
};

#endif // PDFVIEWER_H
