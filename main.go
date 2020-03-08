package main

import
(
	"encoding/json"
	"fmt"
	"flag"
	"os"
	"io/ioutil"
)


func main(){

	hide := flag.NewFlagSet("hide", flag.ExitOnError)
	hideLocationsFile := hide.String("l", "locations.json", "json file of locations")
	hideFenceFile := hide.String("f", "fence.json", "json file with fence data")

	seek := flag.NewFlagSet("seek", flag.ExitOnError)
	seekProofFile := seek.String("p", "proof.json", "json file with the proof")

	if len(os.Args) < 2 {
		fmt.Printf("hide and seek expects 'hide' or 'seek' subcommands")
		os.Exit(1)
	}

	switch os.Args[1] {
	case "hide":
		hide.Parse(os.Args[2:])

		var locations []Location
		var fence Fence

		readJSON(*hideLocationsFile, &locations)
		readJSON(*hideFenceFile, &fence)

		p := Hide(locations, fence)

		bytes, _ := json.Marshal(p)

		ioutil.WriteFile("proof.json", bytes, 0644)

	case "seek":
		seek.Parse(os.Args[2:])
		var proof Proof
		readJSON(*seekProofFile, &proof)
		fmt.Printf("%v\n", proof.Seek())

	default:
		fmt.Printf("expects 'hide' or 'seek' subcommands")
		os.Exit(1)
	}

}

func readJSON(path string, object interface{}) error {
	file, err := os.Open(path)

	defer file.Close()

	if err != nil {
		return fmt.Errorf("open error: %v", err)
	}

	bytes, err := ioutil.ReadAll(file)

	if err != nil {
		return fmt.Errorf("read error: %v", err)
	}

	err = json.Unmarshal(bytes, &object)

	if err != nil {
		return fmt.Errorf("json error: %v", err)
	}

	return nil
}
