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

using namespace std;

Coolrunner2Device::Coolrunner2Device(
	COOLRUNNER2_PART part,
	COOLRUNNER2_PKG pkg,
	COOLRUNNER2_SPEED speed)
	: m_part(part)
	, m_pkg(pkg)
	, m_speed(speed)
{
    // Create ZIA nodes
    for (int i = 0; i < COOLRUNNER2_ZIA_INPUTS[part]; i++) {
        m_zia_nodes.push_back(new Coolrunner2ZIANode(this, i));
    }

    // Create ibuf
    for (int i = 0; i < COOLRUNNER2_NUM_IBUF[part]; i++) {
        m_ibuf.push_back(new Coolrunner2IBuf(this, i));
    }

    // Create obuf
    for (int i = 0; i < COOLRUNNER2_NUM_OBUF[part]; i++) {
        m_obuf.push_back(new Coolrunner2OBuf(this, i));
    }
}

Coolrunner2Device::~Coolrunner2Device()
{
    // Delete ZIA nodes
    for (auto x : m_zia_nodes) {
        delete x;
    }
    m_zia_nodes.clear();

    // Delete ibuf
    for (auto x : m_ibuf) {
        delete x;
    }
    m_ibuf.clear();

    // Delete obuf
    for (auto x : m_obuf) {
        delete x;
    }
    m_obuf.clear();
}

std::string Coolrunner2Device::DebugDump()
{
    string output("CoolRunner-II structure dump\n");

    output += "Part name: ";
    output += COOLRUNNER2_PART_NAMES[this->GetPart()];
    output += "\nPart package: ";
    output += COOLRUNNER2_PKG_NAMES[this->GetPkg()];
    output += "\nPart speed: ";
    output += COOLRUNNER2_SPEED_NAMES[this->GetSpeed()];

    output += "\n\nZIA input nodes:\n";
    for (auto x : m_zia_nodes) {
        output += x->DebugDump();
    }

    output += "\n\nInputs:\n";
    for (auto x : m_ibuf) {
        output += x->DebugDump();
    }

    output += "\n\nOutputs:\n";
    for (auto x : m_obuf) {
        output += x->DebugDump();
    }

    return output;
}
