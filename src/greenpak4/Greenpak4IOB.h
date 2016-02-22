/***********************************************************************************************************************
 * Copyright (C) 2016 Andrew Zonenberg and contributors                                                                *
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
 
#ifndef Greenpak4IOB_h
#define Greenpak4IOB_h

/**
	@brief A single IOB
 */ 
class Greenpak4IOB : public Greenpak4BitstreamEntity
{
public:

	//Construction / destruction
	Greenpak4IOB(
		Greenpak4Device* device,
		unsigned int matrix,
		unsigned int ibase,
		unsigned int oword,
		unsigned int cbase);
	virtual ~Greenpak4IOB();
		
	//Bitfile metadata
	virtual unsigned int GetConfigLen() =0;
	
	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// Accessors for format-dependent bitstream state
	
	//Drive strength for pullup/down resistor
	enum PullStrength
	{
		PULL_10K,
		PULL_100K,
		PULL_1M
	};
	
	//Direction for pullup/down resistor
	enum PullDirection
	{
		PULL_NONE,
		PULL_DOWN,
		PULL_UP
	};
	
	//Drive strength for output
	enum DriveStrength
	{
		DRIVE_1X,
		DRIVE_2X
	};
	
	//Output driver type
	enum DriveType
	{
		DRIVE_PUSHPULL,
		DRIVE_NMOS_OPENDRAIN,
		DRIVE_PMOS_OPENDRAIN,
	};
	
	//Input voltage threshold
	enum InputThreshold
	{
		///Normal digital input
		THRESHOLD_NORMAL,
		
		///Low-voltage digital input
		THRESHOLD_LOW,
		
		///Analog input
		THRESHOLD_ANALOG
	};
	
	virtual void SetSchmittTrigger(bool enabled);
	bool GetSchmittTrigger();
	
	virtual void SetPullStrength(PullStrength strength);
	bool GetPullStrength();
	
	virtual void SetPullDirection(PullDirection direction);
	bool GetPullDirection();
	
	virtual void SetDriveStrength(DriveStrength strength);
	bool GetDriveStrength();
	
	virtual void SetDriveType(DriveType type);
	bool GetDriveType();
	
	virtual void SetInputThreshold(InputThreshold thresh);
	bool GetInputThreshold();
	
protected:

	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// Abstracted version of format-dependent bitstream state
	
	///Set true to enable Schmitt triggering on the input
	bool m_schmittTrigger;
	
	///Strength of the pullup/down resistor, if any
	PullStrength m_pullStrength;
	
	///Direction of the pullup/down resistor, if any
	PullDirection m_pullDirection;
	
	///Strength of the output driver
	DriveStrength m_driveStrength;
	
	///Type of the output driver
	DriveType m_driveType;
	
	///Type of the input threshold
	InputThreshold m_inputThreshold;
};

#endif