cmake_minimum_required(VERSION 3.10.0)
project(cpp-ffi VERSION 0.1.0 LANGUAGES C CXX)

# =============================================================================
# Rust Library Configuration
# =============================================================================
set(RUST_LIB_NAME "rust_ffi")
set(RUST_TARGET_DIR "${CMAKE_SOURCE_DIR}/target")
if (CMAKE_BUILD_TYPE STREQUAL "Debug")
    set(RUST_LIB_DIR "${RUST_TARGET_DIR}/debug")
    message(STATUS "Debug build")
else()
    set(RUST_LIB_DIR "${RUST_TARGET_DIR}/release")
    message(STATUS "Release build")
endif()


if (WIN32)
    set(RUST_LIB_PATH "${RUST_LIB_DIR}/${RUST_LIB_NAME}.dll")
elseif(APPLE)
    set(RUST_LIB_PATH "${RUST_LIB_DIR}/lib${RUST_LIB_NAME}.dylib")
else()
    set(RUST_LIB_PATH "${RUST_LIB_DIR}/lib${RUST_LIB_NAME}.so")
endif()


if (EXISTS ${RUST_LIB_PATH})
    message(STATUS "Rust lib exists")
else()
    message(FATAL_ERROR "Rust lib does not exist: ${RUST_LIB_PATH}. Please build the Rust library first.")
endif()

# =============================================================================
# Executable Target Configuration
# =============================================================================
set(targetname call-rust)
add_executable(${targetname} src/cpp/call_rust.cpp)
target_include_directories(${targetname} PRIVATE ${CMAKE_SOURCE_DIR}/include)


# =============================================================================
# Link Rust Shared Library
# =============================================================================

# link the Rust shared library
if(WIN32) 
    # NOTE: On Windows, there are two types of libraries: the import library (.dll.lib) and the runtime library (.dll)
    target_link_libraries(${targetname} PRIVATE "${RUST_LIB_PATH}.lib")

    # Copy DLL to output directory on Windows
    add_custom_command(TARGET ${targetname} POST_BUILD
        COMMAND ${CMAKE_COMMAND} -E copy
        "${RUST_LIB_PATH}"
        $<TARGET_FILE_DIR:${targetname}>
        COMMENT "Copying ${RUST_LIB_NAME}.dll to output directory"
    )
elseif(APPLE)
    target_link_libraries(${targetname} PRIVATE "${RUST_LIB_PATH}")
else()
    target_link_libraries(${targetname} PRIVATE "${RUST_LIB_PATH}")
endif()

# =============================================================================
# Testing
# =============================================================================
include(CTest)
enable_testing()

# Define test configuration function
function(add_unit_test TARGET_NAME TEST_NAME TEST_TYPE)
    add_test(NAME rust_${TEST_NAME} 
             COMMAND ${TARGET_NAME} ${TEST_TYPE}
             WORKING_DIRECTORY ${CMAKE_CURRENT_BINARY_DIR})
    
    set_tests_properties(rust_${TEST_NAME}
        PROPERTIES 
        TIMEOUT 30
        LABELS "${TARGET_NAME}_unit_tests"
    )
endfunction()

add_unit_test(${targetname} test_add_function "TestRunner::test_add_function")


# =============================================================================
# Packaging
# =============================================================================

set(CPACK_PROJECT_NAME ${PROJECT_NAME})
set(CPACK_PROJECT_VERSION ${PROJECT_VERSION})
include(CPack)

# Platform-specific package generators
if(WIN32)
    set(CPACK_GENERATOR "ZIP")
elseif(APPLE)
    set(CPACK_GENERATOR "Bundle;DragNDrop")
else()
    set(CPACK_GENERATOR "TGZ;DEB")
endif()


install(TARGETS ${targetname} 
        RUNTIME DESTINATION bin
        LIBRARY DESTINATION lib
        ARCHIVE DESTINATION lib/static
)

# install Rust shared library
if(WIN32)
    install(FILES "${RUST_LIB_PATH}" 
            DESTINATION bin)
elseif(APPLE)
    install(FILES "${RUST_LIB_PATH}" 
            DESTINATION lib)
else()
    install(FILES "${RUST_LIB_PATH}" 
            DESTINATION lib)
endif() 

# install include files
install(DIRECTORY ${CMAKE_SOURCE_DIR}/include/ 
        DESTINATION include
        FILES_MATCHING PATTERN "*.h")

