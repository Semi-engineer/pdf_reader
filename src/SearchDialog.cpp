#include "SearchDialog.h"
#include "PDFViewer.h"
#include <QVBoxLayout>
#include <QHBoxLayout>
#include <QMessageBox>

SearchDialog::SearchDialog(PDFViewer *viewer, QWidget *parent)
    : QDialog(parent)
    , m_viewer(viewer)
{
    setWindowTitle(tr("Search in Document"));
    setMinimumSize(500, 400);
    
    // Create widgets
    m_searchEdit = new QLineEdit(this);
    m_searchEdit->setPlaceholderText(tr("Enter search text..."));
    
    m_searchButton = new QPushButton(tr("Search"), this);
    m_searchButton->setDefault(true);
    
    m_statusLabel = new QLabel(this);
    
    m_resultsList = new QListWidget(this);
    
    // Layout
    QHBoxLayout *searchLayout = new QHBoxLayout;
    searchLayout->addWidget(m_searchEdit);
    searchLayout->addWidget(m_searchButton);
    
    QVBoxLayout *mainLayout = new QVBoxLayout(this);
    mainLayout->addLayout(searchLayout);
    mainLayout->addWidget(m_statusLabel);
    mainLayout->addWidget(m_resultsList);
    
    // Connections
    connect(m_searchButton, &QPushButton::clicked, this, &SearchDialog::onSearch);
    connect(m_searchEdit, &QLineEdit::returnPressed, this, &SearchDialog::onSearch);
    connect(m_resultsList, &QListWidget::itemClicked, this, &SearchDialog::onResultClicked);
    
    // Apply dark theme styling
    setStyleSheet(R"(
        QDialog {
            background-color: #2b2b2b;
        }
        QLineEdit {
            background-color: #3c3c3c;
            border: 2px solid #4a4a4a;
            border-radius: 4px;
            padding: 8px;
            color: #ffffff;
            font-size: 14px;
        }
        QLineEdit:focus {
            border-color: #0d7377;
        }
        QPushButton {
            background-color: #0d7377;
            border: none;
            border-radius: 4px;
            padding: 8px 20px;
            color: #ffffff;
            font-weight: bold;
            font-size: 14px;
        }
        QPushButton:hover {
            background-color: #14ffec;
            color: #2b2b2b;
        }
        QPushButton:pressed {
            background-color: #0a5a5d;
        }
        QLabel {
            color: #ffffff;
            font-size: 12px;
        }
        QListWidget {
            background-color: #3c3c3c;
            border: 2px solid #4a4a4a;
            border-radius: 4px;
            color: #ffffff;
            outline: none;
        }
        QListWidget::item {
            padding: 8px;
            border-bottom: 1px solid #4a4a4a;
        }
        QListWidget::item:hover {
            background-color: #4a4a4a;
        }
        QListWidget::item:selected {
            background-color: #0d7377;
        }
    )");
}

SearchDialog::~SearchDialog()
{
}

void SearchDialog::onSearch()
{
    QString searchText = m_searchEdit->text().trimmed();
    
    if (searchText.isEmpty()) {
        QMessageBox::warning(this, tr("Search"), tr("Please enter search text."));
        return;
    }
    
    performSearch();
}

void SearchDialog::performSearch()
{
    m_resultsList->clear();
    
    QString searchText = m_searchEdit->text().trimmed();
    int totalResults = 0;
    
    if (!m_viewer || !m_viewer->hasDocument()) {
        m_statusLabel->setText(tr("No document loaded."));
        return;
    }
    
    int pageCount = m_viewer->pageCount();
    
    for (int i = 0; i < pageCount; ++i) {
        QList<QRectF> results = m_viewer->search(searchText, i);
        
        if (!results.isEmpty()) {
            totalResults += results.count();
            
            QListWidgetItem *item = new QListWidgetItem(m_resultsList);
            item->setText(tr("Page %1 (%2 matches)").arg(i + 1).arg(results.count()));
            item->setData(Qt::UserRole, i);
            item->setIcon(style()->standardIcon(QStyle::SP_FileIcon));
        }
    }
    
    if (totalResults > 0) {
        m_statusLabel->setText(tr("Found %1 results in %2 pages")
                              .arg(totalResults)
                              .arg(m_resultsList->count()));
    } else {
        m_statusLabel->setText(tr("No results found for \"%1\"").arg(searchText));
    }
}

void SearchDialog::onResultClicked(QListWidgetItem *item)
{
    if (item && m_viewer) {
        int pageNumber = item->data(Qt::UserRole).toInt();
        m_viewer->setCurrentPage(pageNumber);
    }
}
