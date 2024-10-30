package main

import (
	"encoding/json"
	"fmt"
	"net/http"
)

type Student struct {
	"student:" string,
	"age:" int,
	"faculty:" string,
	"discripline:" string
}