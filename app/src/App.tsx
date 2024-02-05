import "./styles/App.css";
import { RouterProvider, createBrowserRouter } from "react-router-dom";
import ErrorPage from "./components/pages/ErrorPage";
import { Layout } from "./components/pages/Layout";

const rootRouter = createBrowserRouter([
    {
        path: "/",
        Component: Layout,
        errorElement: <ErrorPage />,
    },
]);

function App() {
    return <RouterProvider router={rootRouter} />;
}

export default App;
