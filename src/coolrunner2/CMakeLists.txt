add_library(coolrunner2 STATIC

	# Post-PAR netlist

	# Unplaced (but techmapped) netlist
)

target_include_directories(coolrunner2
	PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

target_link_libraries(coolrunner2
	json-c xbpar log)
