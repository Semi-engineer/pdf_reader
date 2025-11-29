#ifndef SEARCHDIALOG_H
#define SEARCHDIALOG_H

#include <QDialog>
#include <QLineEdit>
#include <QPushButton>
#include <QListWidget>
#include <QLabel>

class PDFViewer;

class SearchDialog : public QDialog
{
    Q_OBJECT

public:
    explicit SearchDialog(PDFViewer *viewer, QWidget *parent = nullptr);
    ~SearchDialog();

private slots:
    void onSearch();
    void onResultClicked(QListWidgetItem *item);

private:
    void performSearch();
    
    PDFViewer *m_viewer;
    QLineEdit *m_searchEdit;
    QPushButton *m_searchButton;
    QListWidget *m_resultsList;
    QLabel *m_statusLabel;
};

#endif // SEARCHDIALOG_H
