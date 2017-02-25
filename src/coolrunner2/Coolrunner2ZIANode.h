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

#ifndef Coolrunner2ZIANode_h
#define Coolrunner2ZIANode_h

class Coolrunner2Device;

#include <string>

// Class that describes a possible input in the ZIA (either an I/O pin or
// feedback from the PLA/macrocells). Does not correspond to any actual bit in
// the bitstream
class Coolrunner2ZIANode
{
public:
    Coolrunner2ZIANode(
        Coolrunner2Device* device,
        int node_num
        );
    virtual ~Coolrunner2ZIANode();

    std::string DebugDump();

    Coolrunner2Device* GetDevice()
    { return m_device; }

    bool isIO();
    bool isFeedback();

    // Return a virtual I/O pin number for I/O inputs (or -1)
    // This needs a table to map to actual I/O pins but you can look this up in
    // the Coolrunner2Device table
    int getVirtualIONumber();
    // Return the internal macrocell number for feedback inputs (or -1)
    int getInternalMCNumber();

protected:

    Coolrunner2Device* m_device;
    int m_node_num;
};

#endif