import { useState } from "react";
import "./styles/App.css";
import Calculator from "./components/ui/Calculator";
import UserForm from "./components/ui/UserForm";

function App() {
    const [count, setCount] = useState(0);

    return (
        <div className="bg-[hsl(80,50%,50%)] h-screen">
            <UserForm />
            <Calculator />
        </div>
    );
}

export default App;
