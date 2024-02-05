import { MenuCommand } from "./MenuCommand";
import { Footer } from "./Footer";
import { Home } from "./Home";

export function Layout() {
    return (
        <div className="grid gap-4 p-8 bg-light">
            <div className="text-3xl text-center">
                <h1>H2 Sistemas Inteligentes</h1>
            </div>
            <div className="flex-justify-self-center min-w-lg max-w-max">
                <MenuCommand />
            </div>
            <Home />
            <Footer />
        </div>
    );
}
