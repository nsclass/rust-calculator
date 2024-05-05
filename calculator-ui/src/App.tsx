import { useCallback, useState } from "react";

const DisplayCalculation = ({
  calculationResult,
}: {
  calculationResult: any;
}) => {
  const [showTrace, setShowTrace] = useState(false);
  const toggleTrace = useCallback(() => {
    setShowTrace(!showTrace);
  }, [showTrace]);
  return (
    <div className="flex h-auto w-full flex-col">
      <div className="mt-1 mb-1 h-10 content-center rounded-r rounded-l bg-green-700 text-center text-xl text-white">{`Answer: ${calculationResult?.result ?? ""}`}</div>
      <textarea
        className="h-10 w-full border-2"
        value={calculationResult?.trace?.postfix.join(" ")}
      />
      <button className="h-10 w-full border-2 bg-blue-50" onClick={toggleTrace}>
        {showTrace ? "Hide Trace" : "Show Trace"}
      </button>
      {showTrace && (
        <textarea
          className="h-screen w-full border-2"
          value={JSON.stringify(calculationResult, null, 2)}
        />
      )}
    </div>
  );
};

const CalculateText = () => {
  const [calculateText, setCalculateText] = useState<string>("1+2");
  const [calculationResult, setCalculationResult] = useState();

  const calculateRequest = useCallback(async () => {
    const payload = {
      infix: calculateText,
    };

    const response = await fetch("/calculate", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
        // 'Content-Type': 'application/x-www-form-urlencoded',
      },
      body: JSON.stringify(payload),
    });
    const result = await response.json();
    setCalculationResult(result);
  }, [calculateText]);

  return (
    <div className="mt-10 h-auto w-2/3">
      <div className="flex h-10 w-full">
        <input
          className="h-full w-full rounded-l-2xl border-2 text-center"
          type="text"
          placeholder="1 + 2"
          value={calculateText}
          onChange={(e) => setCalculateText(e.target.value)}
        />
        <button
          className="rounded-r-2xl border-2 bg-blue-800 pr-4 pl-2 text-white hover:border-blue-950"
          onClick={calculateRequest}
        >
          Calculate
        </button>
      </div>
      <DisplayCalculation calculationResult={calculationResult} />
    </div>
  );
};

function App() {
  return (
    <div className="flex h-screen justify-center">
      <CalculateText />
    </div>
  );
}

export default App;
