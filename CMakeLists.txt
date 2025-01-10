cmake_minimum_required(VERSION 3.15...3.31)
project(ordo
        VERSION 0.0.1
        DESCRIPTION "Latin language library"
        HOMEPAGE_URL https://git.sr.ht/~donmcc/dice
        LANGUAGES C
)


option(ADDRESS_SANITIZER "Enable the address sanitizer")
if(ADDRESS_SANITIZER)
    add_compile_options(-fsanitize=address -g)
    add_link_options(-fsanitize=address)
endif()

option(BUILD_TESTING "Build the tests" ON)

option(COVERAGE "Enable code coverage analysis")
if(COVERAGE)
    add_compile_options(--coverage -g -O0)
    add_link_options(--coverage)
endif()

option(WALL "Enable all warnings")
if(WALL)
    add_compile_options(-Wall -Werror)
endif()


set(CMAKE_C_STANDARD 99)
enable_testing()
include(GNUInstallDirs)


if(CMAKE_PROJECT_NAME STREQUAL PROJECT_NAME)
    set(is_top_level_project TRUE)
endif()


add_library(ordo STATIC
        ordo/error.h
        ordo/error.c
        ordo/numerus.h
        ordo/numerus.c
        ordo/ordo.h
        ordo/ordo.c
)
target_include_directories(ordo PUBLIC
        $<BUILD_INTERFACE:${CMAKE_CURRENT_SOURCE_DIR}>
        $<INSTALL_INTERFACE:${CMAKE_INSTALL_INCLUDEDIR}>
)


if(is_top_level_project AND BUILD_TESTING)
    add_executable(errores_tests ordo/errorum_probationes.c)
    target_link_libraries(errores_tests ordo)
    add_test(errores_tests errores_tests)

    add_executable(ordo_tests ordo/ordini_probationes.c)
    target_link_libraries(ordo_tests ordo)
    add_test(ordo_tests ordo_tests)

    add_executable(numerus_tests ordo/numeri_probationes.c)
    target_link_libraries(numerus_tests ordo)
    add_test(numerus_tests numerus_tests)
endif()
