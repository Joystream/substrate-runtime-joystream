// Copyright 2019 Joystream Contributors
// This file is part of Joystream node.

// Joystream node is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Joystream node is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Joystream node.  If not, see <http://www.gnu.org/licenses/>.

//! Joystream Node.

#![warn(missing_docs)]

fn main() -> sc_cli::Result<()> {
    #[cfg(not(feature = "standalone"))]
    return joystream_node::command::run();
    #[cfg(feature = "standalone")]
    return joystream_node::command_standalone::run();
}
