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
        ordo/genus.h
        ordo/genus.c
        ordo/linea_mutabilis.c
        ordo/linea_mutabilis.h
        ordo/litterae.h
        ordo/litterae.c
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
    add_executable(erroris_probationes ordo/erroris_probationes.c)
    target_link_libraries(erroris_probationes ordo)
    add_test(erroris_probationes erroris_probationes)

    add_executable(lineae_mutabilis_probationes ordo/lineae_mutabilis_probationes.c)
    target_link_libraries(lineae_mutabilis_probationes ordo)
    add_test(lineae_mutabilis_probationes lineae_mutabilis_probationes)

    add_executable(litterarum_probationes ordo/litterarum_probationes.c)
    target_link_libraries(litterarum_probationes ordo)
    add_test(litterarum_probationes litterarum_probationes)

    add_executable(numeri_probationes ordo/numeri_probationes.c)
    target_link_libraries(numeri_probationes ordo)
    add_test(numeri_probationes numeri_probationes)

    add_executable(ordinis_probationes ordo/ordinis_probationes.c)
    target_link_libraries(ordinis_probationes ordo)
    add_test(ordinis_probationes ordinis_probationes)
endif()
