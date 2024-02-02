import {
    Drawer,
    DrawerClose,
    DrawerContent,
    DrawerDescription,
    DrawerFooter,
    DrawerHeader,
    DrawerTitle,
    DrawerTrigger,
} from "@/components/ui/drawer";
import { Button } from "./button";

export default function Calculator() {
    return (
        <>
            <Drawer>
                <DrawerTrigger>Continuar</DrawerTrigger>
                <DrawerContent>
                    <DrawerHeader>
                        <DrawerTitle>Revise seus dados</DrawerTitle>
                        <DrawerDescription>
                            Seus dados serão analisados após o envio.
                        </DrawerDescription>
                    </DrawerHeader>
                    <DrawerFooter>
                        <Button>Enviar</Button>
                        <DrawerClose>
                            <Button variant="outline">Cancelar</Button>
                        </DrawerClose>
                    </DrawerFooter>
                </DrawerContent>
            </Drawer>
        </>
    );
}
