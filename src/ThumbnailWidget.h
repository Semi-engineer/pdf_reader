#ifndef THUMBNAILWIDGET_H
#define THUMBNAILWIDGET_H

#include <QListWidget>
#include <poppler-qt6.h>

class ThumbnailWidget : public QListWidget
{
    Q_OBJECT

public:
    explicit ThumbnailWidget(QWidget *parent = nullptr);
    ~ThumbnailWidget();
    
    void setDocument(Poppler::Document *document);
    void clear();

signals:
    void pageSelected(int page);

private slots:
    void onItemClicked(QListWidgetItem *item);

private:
    void generateThumbnails();
    
    Poppler::Document *m_document;
};

#endif // THUMBNAILWIDGET_H
