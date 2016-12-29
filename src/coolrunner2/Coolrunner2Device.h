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

#include <string>
#include <vector>

class Coolrunner2Device
{
public:

	Coolrunner2Device(
		COOLRUNNER2_PART part,
		COOLRUNNER2_PKG pkg,
		COOLRUNNER2_SPEED speed);

	virtual ~Coolrunner2Device();

	COOLRUNNER2_PART GetPart()
	{ return m_part; }

	COOLRUNNER2_PKG GetPkg()
	{ return m_pkg; }

	COOLRUNNER2_SPEED GetSpeed()
	{ return m_speed; }

	std::string DebugDump();

	// ZIA NODES (meta)

	Coolrunner2ZIANode* GetZIANode(int i)
	{ return m_zia_nodes[i]; }

	int GetZIANodeCount()
	{ return m_zia_nodes.size(); }

	// Input buffers

	Coolrunner2IBuf* GetIBuf(int i)
	{ return m_ibuf[i]; }

	int GetIBufCount()
	{ return m_ibuf.size(); }

	// Output buffers

	Coolrunner2OBuf* GetOBuf(int i)
	{ return m_obuf[i]; }

	int GetOBufCount()
	{ return m_obuf.size(); }

protected:

	COOLRUNNER2_PART m_part;
	COOLRUNNER2_PKG m_pkg;
	COOLRUNNER2_SPEED m_speed;

	// Owning reference to helper nodes representing ZIA inputs
	std::vector<Coolrunner2ZIANode*> m_zia_nodes;

	// Owning reference to input buffer object
	std::vector<Coolrunner2IBuf*> m_ibuf;

	// Owning reference to output buffer object
	std::vector<Coolrunner2OBuf*> m_obuf;
};

#endif
