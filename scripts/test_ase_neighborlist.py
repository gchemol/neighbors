import ase
import ase.io
import numpy as np

from ase.neighborlist import NeighborList, NewPrimitiveNeighborList

def load_cif_atoms(filename):
    atoms = ase.io.read(filename)
    return atoms

# New NeighborList
def test_new_nl(atoms, cutoff):
    cutoffs = [cutoff/2]*len(atoms)
    nl = NeighborList(cutoffs, skin=0.0, self_interaction=False, bothways=True, primitive=NewPrimitiveNeighborList)
    nl.update(atoms)
    for i in range(len(atoms)):
        (x, _) = nl.get_neighbors(i)
        # break
        # return sorted(x)

# Old NeighborList
def test_nl(atoms, cutoff):
    cutoffs = [cutoff/2]*len(atoms)
    nl = NeighborList(cutoffs, skin=0, bothways=True, self_interaction=False)
    nl.update(atoms)
    for i in range(len(atoms)):
        (x, _) = nl.get_neighbors(i)
        # break
        # return sorted(x)
