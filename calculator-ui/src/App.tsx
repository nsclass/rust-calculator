import { CalculateView } from "./components/CalculationView";
import { TitleBar } from "./components/TitleBar";

function App() {
  return (
    <div className="flex flex-col h-screen items-center">
      <TitleBar />
      <CalculateView />
    </div>
  );
}

export default App;
