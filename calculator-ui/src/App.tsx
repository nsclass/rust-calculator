import {useCallback, useState} from "react";

const DisplayCalculation = ({calculationResult} : { calculationResult: any}) => {
  const [showTrace, setShowTrace] = useState(false)
  const toggleTrace = useCallback(() => {
    setShowTrace(!showTrace)
  }, [showTrace])
  return (
    <div className="flex flex-col w-full h-auto">
      <div
        className="rounded-l rounded-r mt-1 mb-1 h-10 bg-green-700 text-white text-center content-center text-xl">{`Answer: ${calculationResult?.result ?? ""}`}</div>
      <textarea className="border-2 w-full h-10 "
                value={calculationResult?.trace?.postfix.join(" ")}
      />
      <button className="w-full h-10 border-2 bg-blue-50" onClick={toggleTrace}>{showTrace ? "Hide Trace" : "Show Trace"}</button>
      {
        showTrace && <textarea className="border-2 w-full h-screen"
                value={JSON.stringify(calculationResult, null, 2)}
      />}
    </div>
  )
}

const CalculateText = () => {
  const [calculateText, setCalculateText] = useState<string>("1+2")
  const [calculationResult, setCalculationResult] = useState()

  const calculateRequest = useCallback(async () => {

    const payload = {
      infix: calculateText
    }

    const response = await fetch('/calculate', {
      method: 'POST',
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(payload)
    })
    const result = await response.json()
    setCalculationResult(result)
  }, [calculateText])

  return (
    <div className="mt-10 w-2/3 h-auto">
      <div className="flex w-full h-10">
        <input className="border-2 rounded-l-2xl w-full h-full text-center"
               type="text"
               placeholder="1 + 2"
               value={calculateText}
               onChange={e => setCalculateText(e.target.value)}/>
        <button className="border-2 rounded-r-2xl bg-blue-800 text-white pl-2 pr-4 hover:border-blue-950"
                onClick={calculateRequest}>Calculate</button>
      </div>
      <DisplayCalculation calculationResult={calculationResult} />
    </div>
  )
}

function App() {
  return (
    <div className="flex justify-center h-screen">
      <CalculateText/>
    </div>
  )
}

export default App
