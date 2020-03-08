package main

import (
	"github.com/ing-bank/zkrp/bulletproofs"
	"log"
	"math/big"
)

const locAccuracy float64 = 100000.0 // 1.1m error rate is still too much for phone grade gps readings

// Location is plain old location
type Location struct {
	Timestamp int     `json:"ts"`
	Latitude  float64 `json:"lat"`
	Longitude float64 `json:"lng"`
}

// Fence is a rectangular geo-fence(region) with a time window
type Fence struct {
	TsRange  [2]int     `json:"timestamp"`
	LatRange [2]float64 `json:"latitude"`
	LngRange [2]float64 `json:"longitude"`
}

// Proof is a combination of Bulletproofs of timestamp, latitude and longitude
type Proof struct {
	TsProof  bulletproofs.ProofBPRP `json:"ts"`
	LatProof bulletproofs.ProofBPRP `json:"lat"`
	LngProof bulletproofs.ProofBPRP `json:"lng"`
}

// Seek verifies the proof
func (p *Proof) Seek() bool {

	ts := verify(p.TsProof)
	lat := verify(p.LatProof)
	lng := verify(p.LngProof)

	return ts && lat && lng
}

// Hide creates a proof (hides the information)
func Hide(l []Location, f Fence) Proof {

	p := Proof{}

	for _, loc := range l {
		if check(loc, f) {
			// create params
			p.TsProof = generateProof(loc.Timestamp, f.TsRange[0], f.TsRange[1])
			p.LatProof = generateProof(f2i(loc.Latitude), f2i(f.LatRange[0]), f2i(f.LatRange[1]))
			p.LngProof = generateProof(f2i(loc.Longitude), f2i(f.LngRange[0]), f2i(f.LngRange[1]))
			return p
		}
	}

	// TODO use threads?
	p.TsProof = generateProof(0, f.TsRange[0], f.TsRange[1])
	p.LatProof = generateProof(-90, f2i(f.LatRange[0]), f2i(f.LatRange[1]))
	p.LngProof = generateProof(-180, f2i(f.LngRange[0]), f2i(f.LngRange[1]))

	return p
}

func verify(p bulletproofs.ProofBPRP) bool {
	ok, err := p.Verify()

	if err != nil {
		log.Fatal("could not verify range proof")
	}
	return ok
}

func generateProof(secret, a, b int) bulletproofs.ProofBPRP {
	// make params

	params, err := bulletproofs.SetupGeneric(int64(a), int64(b))
	if err != nil {
		log.Fatalf("could not create range params for %v - %v", a, b)
	}

	bigSecret := new(big.Int).SetInt64(int64(secret))

	proof, err := bulletproofs.ProveGeneric(bigSecret, params)

	if err != nil {
		log.Fatalf("could not create proof for %v", secret)
	}

	return proof
}

func check(l Location, f Fence) (isInside bool) {
	isInsideTs := f.TsRange[0] < l.Timestamp && l.Timestamp < f.TsRange[1]
	isInsideLat := f.LatRange[0] < l.Latitude && l.Latitude < f.LatRange[1]
	isInsideLng := f.LngRange[0] < l.Longitude && l.Longitude < f.LngRange[1]

	isInside = isInsideTs && isInsideLat && isInsideLng

	return
}

// converts lat lng float64 values to int to be a comparable range
func f2i(n float64) int {
	return int(n * locAccuracy)
}
