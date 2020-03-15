mod fence;
mod location;
mod singlebulletproof;

use singlebulletproof::SingleBulletProof;
use location::Location;
use fence::Fence;
use chrono::prelude::*;
use chrono::Duration;
use std::io::Read;


#[cfg(test)]
mod tests;
