package mr

import "fmt"
import "log"
import "net/rpc"
import "hash/fnv"
import "os"
import "io/ioutil"
import "encoding/json"
import "time"

//
// Map functions return a slice of KeyValue.
//
type KeyValue struct {
	Key   string
	Value string
}

//
// use ihash(key) % NReduce to choose the reduce
// task number for each KeyValue emitted by Map.
//
func ihash(key string) int {
	h := fnv.New32a()
	h.Write([]byte(key))
	return int(h.Sum32() & 0x7fffffff)
}

func handleMap(filename string, mapNo, totalReduce int, mapf func(string, string) []KeyValue) {
	file, _ := os.Open(filename)
	defer file.Close()

	content, _ := ioutil.ReadAll(file)
	kvs := mapf(filename, string(content))

	kvsByHash := map[int][]KeyValue{}

	for _, kv := range kvs {
		hash := ihash(kv.Key) % totalReduce
		if k, found := kvsByHash[hash]; found {
			k = append(k, kv)
			kvsByHash[hash] = k
		} else {
			kvsByHash[hash] = []KeyValue{kv}
		}
	}

	for hash, kvs := range kvsByHash {
		filename := fmt.Sprintf("mr-%d-%d", mapNo, hash)
		os.Remove(filename)
		out, _ := os.Create(filename)
		defer out.Close()
		encoder := json.NewEncoder(out)
		encoder.Encode(kvs)
	}
}

func handleReduce(reduceNo, totalMap int, reducef func(string, []string) string) {
	valueByKey := map[string][]string{}
	for mapNo := 0; mapNo < totalMap; mapNo++ {
		in, err := os.Open(fmt.Sprintf("mr-%d-%d", mapNo, reduceNo))
		if err != nil {
			continue
		}
		defer in.Close()

		var kvs []KeyValue
		decoder := json.NewDecoder(in)
		decoder.Decode(&kvs)

		for _, kv := range kvs {
			if v, found := valueByKey[kv.Key]; found {
				v = append(v, kv.Value)
				valueByKey[kv.Key] = v
			} else {
				valueByKey[kv.Key] = []string{kv.Value}
			}
		}
	}

	outName := fmt.Sprintf("mr-out-%d", reduceNo)
	os.Remove(outName)
	out, _ := os.Create(outName)
	defer out.Close()
	for k, v := range valueByKey {
		out.WriteString(fmt.Sprintf("%s %s\n", k, reducef(k, v)))
	}
}

//
// main/mrworker.go calls this function.
//
func Worker(mapf func(string, string) []KeyValue, reducef func(string, []string) string) {

	// Your worker implementation here.

	// uncomment to send the Example RPC to the master.
	// CallExample()

	for {
		args := RPCArgs{}
		reply := RPCReply{}

		args.ReqType = ReqTypeRequestJob
		call("Master.Handle", &args, &reply)

		if reply.JobType == JobTypeNone {
			return
		}

		if reply.JobType == JobTypeMap {
			handleMap(reply.File, reply.MapNo, reply.TotalReduce, mapf)
			call("Master.Handle", &RPCArgs{
				ReqType:  ReqTypeDoneMapJob,
				MapNo:    reply.MapNo,
				CheckSum: reply.CheckSum,
			}, &RPCReply{})
		}

		if reply.JobType == JobTypeReduce {
			handleReduce(reply.ReduceNo, reply.TotalMap, reducef)
			call("Master.Handle", &RPCArgs{
				ReqType:  ReqTypeDoneReduceJob,
				ReduceNo: reply.ReduceNo,
				CheckSum: reply.CheckSum,
			}, &RPCReply{})
		}

		time.Sleep(time.Second * 1)
	}
}

//
// example function to show how to make an RPC call to the master.
//
// the RPC argument and reply types are defined in rpc.go.
//
func CallExample() {

	// declare an argument structure.
	args := ExampleArgs{}

	// fill in the argument(s).
	args.X = 99

	// declare a reply structure.
	reply := ExampleReply{}

	// send the RPC request, wait for the reply.
	call("Master.Example", &args, &reply)

	// reply.Y should be 100.
	fmt.Printf("reply.Y %v\n", reply.Y)
}

//
// send an RPC request to the master, wait for the response.
// usually returns true.
// returns false if something goes wrong.
//
func call(rpcname string, args interface{}, reply interface{}) bool {
	// c, err := rpc.DialHTTP("tcp", "127.0.0.1"+":1234")
	c, err := rpc.DialHTTP("unix", "mr-socket")
	if err != nil {
		log.Fatal("dialing:", err)
	}
	defer c.Close()

	err = c.Call(rpcname, args, reply)
	if err == nil {
		return true
	}

	fmt.Println(err)
	return false
}
