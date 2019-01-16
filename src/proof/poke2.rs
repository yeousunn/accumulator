// TODO
use super::PoKE2;
use num::BigUint;

pub fn compute_poke2<G>(base: G, exp: BigUint, result: G) -> PoKE2<G> {
  PoKE2 {
    z: base,
    q: result,
    r: exp,
  }
}
