cmake_minimum_required(VERSION 3.16)
project(PDFReader VERSION 1.0.0 LANGUAGES CXX)

set(CMAKE_CXX_STANDARD 17)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

set(CMAKE_AUTOMOC ON)
set(CMAKE_AUTORCC ON)
set(CMAKE_AUTOUIC ON)

# Find Qt6
find_package(Qt6 REQUIRED COMPONENTS Core Gui Widgets)

# Find Poppler-Qt6
find_package(Poppler COMPONENTS Qt6 REQUIRED)

# Source files
set(SOURCES
    src/main.cpp
    src/MainWindow.cpp
    src/PDFViewer.cpp
    src/ThumbnailWidget.cpp
    src/SearchDialog.cpp
)

set(HEADERS
    src/MainWindow.h
    src/PDFViewer.h
    src/ThumbnailWidget.h
    src/SearchDialog.h
)

# Create executable
add_executable(${PROJECT_NAME} ${SOURCES} ${HEADERS})

# Link libraries
target_link_libraries(${PROJECT_NAME}
    Qt6::Core
    Qt6::Gui
    Qt6::Widgets
    Poppler::Qt6
)

# Include directories
target_include_directories(${PROJECT_NAME} PRIVATE
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)

# Set output directory
set_target_properties(${PROJECT_NAME} PROPERTIES
    RUNTIME_OUTPUT_DIRECTORY ${CMAKE_BINARY_DIR}/bin
)
