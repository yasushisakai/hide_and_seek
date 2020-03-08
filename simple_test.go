package main

import (
	"log"
	"encoding/json"
	"io/ioutil"
	"os"
	"testing"
)

func TestSimpleInside(t *testing.T) {

	log.Println("*--* simple inside test *--*")

	loc := Location{}
	loc.Timestamp = 1583607600 // March 7th, 2020, 2pm, EST
	loc.Latitude = 42.360434   // more or less the Media Lab
	loc.Longitude = -71.087223

	locs := make([]Location, 0)
	locs = append(locs, loc)

	fence := Fence{}
	fence.TsRange = [2]int{1583604000, 1583611200}
	fence.LatRange = [2]float64{42.3602, 42.3606}
	fence.LngRange = [2]float64{-71.087500, -71.087000}

	p := Hide(locs, fence)

	if !p.Seek() {
		t.Errorf("loc %v should be inside fence %v", loc, fence)
	}
}

func TestSimpleOutside(t *testing.T) {

	log.Println("*--* simple outside test *--*")

	loc := Location{}
	loc.Timestamp = 1583607600 // March 7th, 2020, 2pm, EST
	// huge triangle in arizona
	loc.Latitude = 33.7462611
	loc.Longitude = -112.6370824

	locs := make([]Location, 0)
	locs = append(locs, loc)

	fence := Fence{}
	fence.TsRange = [2]int{1583604000, 1583611200}
	fence.LatRange = [2]float64{42.3602, 42.3606}
	fence.LngRange = [2]float64{-71.087500, -71.087000}

	p := Hide(locs, fence)

	if p.Seek() {
		t.Errorf("loc %v should be outside fence %v", loc, fence)
	}
}

func TestJSON(t *testing.T) {

	log.Println("*--* JSON test *--*")

	var locations []Location
	var fence Fence

	readJSON("locations.json", &locations)
	readJSON("fence.json", &fence)

	p := Hide(locations, fence)

	if !p.Seek() {
		t.Errorf("at least one location should be included inside the fence")
	}

}


func readJSON(path string, object interface{}) interface{} {
	file, err := os.Open(path)

	defer file.Close()

	if err != nil {
		log.Fatalf("open error: %v", err)
	}

	bytes, err := ioutil.ReadAll(file)

	if err != nil {
		log.Fatalf("read error: %v", err)
	}

	err = json.Unmarshal(bytes, &object)

	if err != nil {
		log.Fatalf("json error: %v", err)
	}

	return object

}
