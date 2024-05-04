import {useState} from "react";

const CalculateText = () => {
  const [calculateText, setCalculateText] = useState<string>("1+2")
  const [calculationResult, setCalculationResult] = useState({})

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
    <div className="w-1/2 h-20">
      <div className="flex w-full h-1/2">
        <input className="border-2 rounded-l-2xl w-full h-full text-center" type="text" placeholder="1 + 2"
               value={calculateText}
               onChange={e => setCalculateText(e.target.value)}/>
        <button className="border-2 rounded-r-2xl" onClick={calculateRequest}>Calculate</button>
      </div>
      <div className="flex w-full h-full">
        <textarea className="border-2 w-full "
                  value={JSON.stringify(calculationResult, null, 2)}

        />
      </div>
    </div>
  )
}

function App() {
  return (
    <div className="flex justify-center h-screen items-center">
      <CalculateText/>
    </div>
  )
}

export default App
