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

#include <Coolrunner2.h>
#include <cassert>
#include <log.h>

using namespace std;

Coolrunner2ZIANode::Coolrunner2ZIANode(
    Coolrunner2Device* device,
    int node_num
    )
    : m_device(device)
    , m_node_num(node_num)
{

}

Coolrunner2ZIANode::~Coolrunner2ZIANode()
{

}

bool Coolrunner2ZIANode::isIO()
{
    switch (m_device->GetPart()) {
        case COOLRUNNER2_XC2C32:
        case COOLRUNNER2_XC2C32A:
            return m_node_num <= 32;
        case COOLRUNNER2_XC2C64:
        case COOLRUNNER2_XC2C64A:
            return m_node_num <= 63;
        case COOLRUNNER2_XC2C128:
            return m_node_num <= 99;
        case COOLRUNNER2_XC2C256:
            return m_node_num <= 183;
        case COOLRUNNER2_XC2C384:
            return m_node_num <= 239;
        case COOLRUNNER2_XC2C512:
            return m_node_num <= 269;
        default:
            // Can't happen
            LogError("Internal error\n");
            assert(false);
    }
}

bool Coolrunner2ZIANode::isFeedback()
{
    // Hopefully no exceptions to this?
    return !isIO();
}

std::string Coolrunner2ZIANode::DebugDump()
{
    std::string output("ZIA #");
    output += to_string(m_node_num);

    if (isIO()) {
        output += "\n Input from IO #";
        output += to_string(getVirtualIONumber());
        output += "\n";
    }
    if (isFeedback()) {
        output += "\n Feedback from macrocell #";
        output += to_string(getInternalMCNumber());
        output += "\n";
    }

    return output;
}

int Coolrunner2ZIANode::getVirtualIONumber()
{
    if (!isIO())
        return -1;

    if (m_device->GetPart() == COOLRUNNER2_XC2C32 || m_device->GetPart() == COOLRUNNER2_XC2C32A) {
        // Special case for the input-only pin to make it virtual pin 33
        if (m_node_num <= 15) {
            return m_node_num;
        } else if (m_node_num == 16) {
            return 32;
        } else {
            return m_node_num - 1;
        }
    } else {
        return m_node_num;
    }
}

int Coolrunner2ZIANode::getInternalMCNumber()
{
    if (!isFeedback())
        return -1;

    switch (m_device->GetPart()) {
        case COOLRUNNER2_XC2C32:
        case COOLRUNNER2_XC2C32A:
            return m_node_num - 33;
        case COOLRUNNER2_XC2C64:
        case COOLRUNNER2_XC2C64A:
            return m_node_num - 64;
        case COOLRUNNER2_XC2C128:
            return m_node_num - 100;
        case COOLRUNNER2_XC2C256:
            return m_node_num - 184;
        case COOLRUNNER2_XC2C384:
            return m_node_num - 240;
        case COOLRUNNER2_XC2C512:
            return m_node_num - 270;
        default:
            // Can't happen
            LogError("Internal error\n");
            assert(false);
    }
}
