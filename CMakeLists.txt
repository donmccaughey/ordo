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
    src/ordo.h
    src/ordo.c
)


if(is_top_level_project AND BUILD_TESTING)
    add_executable(ordo_tests src/ordo_tests.c)
    target_link_libraries(ordo_tests ordo)
    add_test(ordo_tests ordo_tests)
endif()
