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

`default_nettype none

module Analog(bg_ok, vref_out, vin, cout);

	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// I/O declarations
	
	(* LOC = "P20" *)
	output wire bg_ok;
	
	(* LOC = "P19" *)
	(* IBUF_TYPE = "ANALOG" *)
	output wire vref_out;
	
	(* LOC = "P6" *)
	(* IBUF_TYPE = "ANALOG" *)
	input wire vin;
	
	(* LOC = "P18" *)
	output wire cout;

	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// System reset stuff

	//Power-on reset
	wire por_done;
	GP_POR #(
		.POR_TIME(500)
	) por (
		.RST_DONE(por_done)
	);

	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// Oscillators
		
	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// 1.0V bandgap voltage reference (used by a lot of the mixed signal IP)
	
	wire bandgap_vout;
	GP_BANDGAP #(
		.AUTO_PWRDN(0),
		.CHOPPER_EN(1),
		.OUT_DELAY(550)
	) bandgap (
		.OK(bg_ok),
		.VOUT(bandgap_vout)
	);
	
	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// Voltage reference driving a comparator and an external pin
	
	GP_VREF #(
		.VIN_DIV(4'd1),
		.VREF(16'd750)
	) vref (
		.VIN(1'b0),
		.VOUT(vref_out)
	);
	
	////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
	// Comparator checking vin against the reference
	
	GP_ACMP #(
		.BANDWIDTH("LOW"),
		.VIN_ATTEN(1'b1),
		.VIN_ISRC_EN(1'b0),
		.HYSTERESIS(8'd50)
	) comparator (
		.PWREN(1'b1),
		.OUT(cout),
		.VIN(vin),
		.VREF(vref_out)
	);
	
endmodule
