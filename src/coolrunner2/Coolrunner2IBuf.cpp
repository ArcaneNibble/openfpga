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

Coolrunner2IBuf::Coolrunner2IBuf(
    Coolrunner2Device* device,
    int num
    )
    : m_device(device)
    , m_internal_num(num)
{
    m_schmitt_trigger = false;
    m_pull_up = false;
}

Coolrunner2IBuf::~Coolrunner2IBuf()
{

}

std::string Coolrunner2IBuf::DebugDump()
{
    std::string output("INPUT #");
    output += to_string(m_internal_num);

    output += "\n Schmitt trigger: ";
    output += m_schmitt_trigger ? "YES" : "NO";

    output += "\n Pull-up: ";
    output += m_pull_up ? "YES" : "NO";
    output += "\n";

    return output;
}
