add_library(coolrunner2 STATIC

	# Post-PAR netlist
	Coolrunner2BitstreamEntity.cpp
	Coolrunner2Device.cpp
	Coolrunner2FB.cpp
	Coolrunner2IBuf.cpp
	Coolrunner2MCFF.cpp
	Coolrunner2MCXorGate.cpp
	Coolrunner2OBuf.cpp
	Coolrunner2PTerm.cpp
	Coolrunner2STerm.cpp
	Coolrunner2ZIANode.cpp

	# Unplaced (but techmapped) netlist
)

target_include_directories(coolrunner2
	PUBLIC ${CMAKE_CURRENT_SOURCE_DIR})

target_link_libraries(coolrunner2
	json-c xbpar log)
