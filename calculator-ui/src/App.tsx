import {useState} from "react";

const DisplayCalculation = ({calculationResult} : { calculationResult: any}) => {
  return (
    <div className="flex flex-col w-full h-auto">
      <label className="h-10">{`Result: ${calculationResult?.result ?? ""}`}</label>
        <textarea className="border-2 w-full h-screen "
                  value={JSON.stringify(calculationResult, null, 2)}
        />
    </div>
  )
}

const CalculateText = () => {
  const [calculateText, setCalculateText] = useState<string>("1+2")
  const [calculationResult, setCalculationResult] = useState()

  const calculateRequest = async () => {

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
  }

  return (
    <div className="mt-10 w-2/3 h-auto">
      <div className="flex w-full h-10">
        <input className="border-2 rounded-l-2xl w-full h-full text-center" type="text" placeholder="1 + 2"
               value={calculateText}
               onChange={e => setCalculateText(e.target.value)}/>
        <button className="border-2 rounded-r-2xl" onClick={calculateRequest}>Calculate</button>
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
