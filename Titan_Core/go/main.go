package main

import (
	"crypto/sha256"
	"encoding/hex"
	"fmt"
	"os"
	"path/filepath"
	"strings"
	"time"
)

func hash(b []byte) string {
	h := sha256.Sum256(b)
	return hex.EncodeToString(h[:])
}

func main() {
	cwd, _ := os.Getwd()
	jobs := filepath.Join(cwd, "jobs")
	out := filepath.Join(cwd, "out")
	os.MkdirAll(jobs, 0755)
	os.MkdirAll(out, 0755)

	fmt.Println("⚡ TITAN GO BUS ONLINE (WINDOWS)")
	fmt.Println("   Watching:", jobs)

	for {
		files, _ := os.ReadDir(jobs)
		for _, f := range files {
			if f.IsDir() || !strings.HasSuffix(f.Name(), ".txt") {
				continue
			}

			p := filepath.Join(jobs, f.Name())
			data, err := os.ReadFile(p)
			if err != nil { continue }

			h := hash(data)
			outFile := filepath.Join(out, f.Name()+"."+h[:8]+".bus")
			
			// Stamp output
			os.WriteFile(outFile, data, 0644)
			
			// Archive job
			os.Rename(p, p+".done")
			
			fmt.Println("   Processed:", f.Name())
		}
		time.Sleep(500 * time.Millisecond)
	}
}
