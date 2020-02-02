package mr

//
// RPC definitions.
//
// remember to capitalize all names.
//

type JobType int

var (
	JobTypeNone   JobType = 0
	JobTypeMap    JobType = 1
	JobTypeReduce JobType = 2
	JobTypeRetry  JobType = 3
)

type ReqType int

var (
	ReqTypeRequestJob    ReqType = 0
	ReqTypeDoneMapJob    ReqType = 1
	ReqTypeDoneReduceJob ReqType = 2
)

type RPCArgs struct {
	ReqType  ReqType
	MapNo    int
	ReduceNo int
	CheckSum int
}

type RPCReply struct {
	File        string
	JobType     JobType
	TotalReduce int
	ReduceNo    int
	TotalMap    int
	MapNo       int
	CheckSum    int
}

//
// example to show how to declare the arguments
// and reply for an RPC.
//

type ExampleArgs struct {
	X int
}

type ExampleReply struct {
	Y int
}

// Add your RPC definitions here.
