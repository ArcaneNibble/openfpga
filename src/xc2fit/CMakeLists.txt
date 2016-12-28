add_executable(xc2fit
	main.cpp
)

target_link_libraries(xc2fit
	coolrunner2 xbpar log)

install(TARGETS xc2fit
    RUNTIME DESTINATION ${CMAKE_INSTALL_BINDIR})
