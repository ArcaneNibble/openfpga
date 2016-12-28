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

#ifndef Coolrunner2Device_h
#define Coolrunner2Device_h

class Coolrunner2Device
{
public:

	enum COOLRUNNER2_PART
	{
		COOLRUNNER2_XC2C32,
		COOLRUNNER2_XC2C32A,
		COOLRUNNER2_XC2C64,
		COOLRUNNER2_XC2C64A,
		COOLRUNNER2_XC2C128,
		COOLRUNNER2_XC2C256,
		COOLRUNNER2_XC2C384,
		COOLRUNNER2_XC2C512,

		COOLRUNNER2_PART_COUNT
	};

	// PB-free parts are omitted when equivalent to non-PB-free
	enum COOLRUNNER2_PKG
	{
		COOLRUNNER2_PKG_QFG32,
		COOLRUNNER2_PKG_VQ44,
		COOLRUNNER2_PKG_QFG48,
		COOLRUNNER2_PKG_CP56,
		COOLRUNNER2_PKG_VQ100,
		COOLRUNNER2_PKG_CP132,
		COOLRUNNER2_PKG_TQ144,
		COOLRUNNER2_PKG_PQ208,
		COOLRUNNER2_PKG_FT256,
		COOLRUNNER2_PKG_FG324,

		COOLRUNNER2_PKG_COUNT
	};

	enum COOLRUNNER2_SPEED
	{
		COOLRUNNER2_SPEED_3,
		COOLRUNNER2_SPEED_4,
		COOLRUNNER2_SPEED_5,
		COOLRUNNER2_SPEED_6,
		COOLRUNNER2_SPEED_7,
		COOLRUNNER2_SPEED_10,

		COOLRUNNER2_SPEED_COUNT
	};

	Coolrunner2Device(
		Coolrunner2Device::COOLRUNNER2_PART part,
		Coolrunner2Device::COOLRUNNER2_PKG pkg,
		Coolrunner2Device::COOLRUNNER2_SPEED speed);

	virtual ~Coolrunner2Device();

	COOLRUNNER2_PART GetPart()
	{ return m_part; }

	COOLRUNNER2_PKG GetPkg()
	{ return m_pkg; }

	COOLRUNNER2_SPEED GetSpd()
	{ return m_speed; }

protected:

	COOLRUNNER2_PART m_part;
	COOLRUNNER2_PKG m_pkg;
	COOLRUNNER2_SPEED m_speed;
};

// FIXME: C++ is hard and some static/constexpr thing is needed
const char * const COOLRUNNER2_PART_NAMES[Coolrunner2Device::COOLRUNNER2_PART_COUNT] = {
	"XC2C32",
	"XC2C32A",
	"XC2C64",
	"XC2C64A",
	"XC2C128",
	"XC2C256",
	"XC2C384",
	"XC2C512"
};

const char * const COOLRUNNER2_PKG_NAMES[Coolrunner2Device::COOLRUNNER2_PKG_COUNT] = {
	"QFG32",
	"VQ44",
	"QFG48",
	"CP56",
	"VQ100",
	"CP132",
	"TQ144",
	"PQ208",
	"FT256",
	"FG324"
};

const char * const COOLRUNNER2_SPEED_NAMES[Coolrunner2Device::COOLRUNNER2_SPEED_COUNT] = {
	"3",
	"4",
	"5",
	"6",
	"7",
	"10"
};

#endif
