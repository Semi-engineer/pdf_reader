#include <QApplication>
#include "MainWindow.h"

int main(int argc, char *argv[])
{
    QApplication app(argc, argv);
    
    app.setApplicationName("PDF Reader");
    app.setApplicationVersion("1.0.0");
    app.setOrganizationName("PDFReader");
    
    MainWindow window;
    window.show();
    
    // If a file path is provided as argument, open it
    if (argc > 1) {
        window.openFile(QString::fromLocal8Bit(argv[1]));
    }
    
    return app.exec();
}
