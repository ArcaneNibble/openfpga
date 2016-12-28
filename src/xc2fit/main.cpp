/***********************************************************************************************************************
 * Copyright (C) 2016 Robert Ou and contributors                                                                *
 *                                                                                                                     *
 * This program is free software; you can redistribute it and/or modify it under the terms of the GNU Lesser General   *
 * Public License as published by the Free Software Foundation; either version 2.1 of the License, or (at your option) *
 * any later version.                                                                                                  *
 *                                                                                                                     *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied  *
 * warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for     *
 * more details.                                                                                                       *
 *                                                                                                                     *
 * You should have received a copy of the GNU Lesser General Public License along with this program; if not, you may   *
 * find one here:                                                                                                      *
 * https://www.gnu.org/licenses/old-licenses/lgpl-2.1.txt                                                              *
 * or you may search the http://www.gnu.org website for the version 2.1 license, or you may write to the Free Software *
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301, USA                                      *
 **********************************************************************************************************************/

#include "xc2fit.h"

using namespace std;

int main(int argc, char* argv[])
{
	Severity console_verbosity = Severity::NOTICE;

	//Netlist file
	string fname = "";

	//Output file
	string ofname = "";

	// Part name, package, and speed grade
	Coolrunner2Device::COOLRUNNER2_PART part = Coolrunner2Device::COOLRUNNER2_XC2C32A;
	Coolrunner2Device::COOLRUNNER2_PKG pkg = Coolrunner2Device::COOLRUNNER2_PKG_VQ44;
	Coolrunner2Device::COOLRUNNER2_SPEED speed = Coolrunner2Device::COOLRUNNER2_SPEED_6;

	//Parse command-line arguments
	for(int i=1; i<argc; i++)
	{
		string s(argv[i]);

		//Let the logger eat its args first
		if(ParseLoggerArguments(i, argc, argv, console_verbosity))
			continue;

		else if(s == "--help")
		{
			ShowUsage();
			return 0;
		}
		else if(s == "--version")
		{
			ShowVersion();
			return 0;
		}
		else if(s == "-o" || s == "--output")
		{
			if(i+1 < argc)
				ofname = argv[++i];
			else
			{
				printf("--output requires an argument\n");
				return 1;
			}
		}
		else if( (s == "--part") || (s == "-p") )
		{
			if(i+1 < argc)
			{
				bool ret = ParsePartName(part, pkg, speed, argv[++i]);
				if (!ret) {
					printf("invalid part\n");
					return 1;
				}
			}
			else
			{
				printf("--part requires an argument\n");
				return 1;
			}
		}

		//assume it's the netlist file if it'[s the first non-switch argument
		else if( (s[0] != '-') && (fname == "") )
			fname = s;

		else
		{
			printf("Unrecognized command-line argument \"%s\", use --help\n", s.c_str());
			return 1;
		}
	}

	//Netlist filenames must be specified
	if( (fname == "") || (ofname == "") )
	{
		ShowUsage();
		return 1;
	}

	//Set up logging
	g_log_sinks.emplace(g_log_sinks.begin(), new STDLogSink(console_verbosity));

	//Print header
	if(console_verbosity >= Severity::NOTICE)
		ShowVersion();

	//Print configuration
	LogNotice("\nDevice configuration:\n");
	{
		LogIndenter li;

		LogNotice("Target part:     %s\n", COOLRUNNER2_PART_NAMES[part]);
		LogNotice("Target package:  %s\n", COOLRUNNER2_PKG_NAMES[pkg]);
		LogNotice("Target speed:    %s\n", COOLRUNNER2_SPEED_NAMES[speed]);
	}

	// Create the device data stuctures
	Coolrunner2Device device(part, pkg, speed);
}

void ShowUsage()
{
	printf(//                                                                               v 80th column
		"Usage: xc2fit -p part -o bitstream.txt netlist.json\n"
		"    --debug\n"
		"        Prints lots of internal debugging information.\n"
		"    -l, --logfile        <file>\n"
		"        Causes verbose log messages to be written to <file>.\n"
		"    -L, --logfile-lines  <file>\n"
		"        Causes verbose log messages to be written to <file>, flushing after\n"
		"        each line.\n"
		"    -o, --output         <bitstream>\n"
		"        Writes bitstream into the specified file.\n"
		"    -p, --part\n"
		"        Specifies the part to target (name-speed-pkg)\n"
		"    -q, --quiet\n"
		"        Causes only warnings and errors to be written to the console.\n"
		"        Specify twice to also silence warnings.\n"
		"    --verbose\n"
		"        Prints additional information about the design.\n");
}

void ShowVersion()
{
	printf(
		"CoolRunner-II fitter by Robert Ou.\n"
		"\n"
		"License: LGPL v2.1+\n"
		"This is free software: you are free to change and redistribute it.\n"
		"There is NO WARRANTY, to the extent permitted by law.\n");
}

bool ParsePartName(
	Coolrunner2Device::COOLRUNNER2_PART &part,
	Coolrunner2Device::COOLRUNNER2_PKG &pkg,
	Coolrunner2Device::COOLRUNNER2_SPEED &speed,
	const char *name)
{
	char *name_part = NULL, *name_pkg = NULL, *name_speed = NULL;
	const char *orig_name;

	orig_name = name;
	name = strchr(name, '-');
	if (!name) {
		printf("Malformed part name\n");
		goto fail;
	}
	name_part = strndup(orig_name, name - orig_name);
	name++;

	orig_name = name;
	name = strchr(name, '-');
	if (!name) {
		printf("Malformed part name\n");
		goto fail;
	}
	name_speed = strndup(orig_name, name - orig_name);
	name_pkg = strdup(name + 1);

	int i;
	for (i = 0; i < Coolrunner2Device::COOLRUNNER2_PART_COUNT; i++) {
		if (strcmp(name_part, COOLRUNNER2_PART_NAMES[i]) == 0) {
			break;
		}
	}
	if (i == Coolrunner2Device::COOLRUNNER2_PART_COUNT) {
		printf("Bad part name %s\n", name_part);
		goto fail;
	}
	part = (Coolrunner2Device::COOLRUNNER2_PART)i;

	for (i = 0; i < Coolrunner2Device::COOLRUNNER2_PKG_COUNT; i++) {
		if (strcmp(name_pkg, COOLRUNNER2_PKG_NAMES[i]) == 0) {
			break;
		}
	}
	if (i == Coolrunner2Device::COOLRUNNER2_PKG_COUNT) {
		printf("Bad package name %s\n", name_pkg);
		goto fail;
	}
	pkg = (Coolrunner2Device::COOLRUNNER2_PKG)i;

	for (i = 0; i < Coolrunner2Device::COOLRUNNER2_SPEED_COUNT; i++) {
		if (strcmp(name_speed, COOLRUNNER2_SPEED_NAMES[i]) == 0) {
			break;
		}
	}
	if (i == Coolrunner2Device::COOLRUNNER2_SPEED_COUNT) {
		printf("Bad speed grade %s\n", name_speed);
		goto fail;
	}
	speed = (Coolrunner2Device::COOLRUNNER2_SPEED)i;

	free(name_part);
	free(name_pkg);
	free(name_speed);

	// Validate the combination
	if (!COOLRUNNER2_VALID_COMBINATIONS[part][pkg][speed]) {
		printf("Bad combination of part/package/speed\n");
		return false;
	}

	return true;

fail:
	free(name_part);
	free(name_pkg);
	free(name_speed);

	return false;
}
