import {
    EnvelopeClosedIcon,
    GearIcon,
    PersonIcon,
} from "@radix-ui/react-icons";

import {
    Command,
    CommandEmpty,
    CommandGroup,
    CommandInput,
    CommandItem,
    CommandList,
    CommandSeparator,
    CommandShortcut,
} from "@/components/ui/command";
import { CalculatorIcon } from "lucide-react";

export function MenuCommand() {
    return (
        <Command className="rounded-lg border shadow-md">
            <CommandInput placeholder="Digite um comando ou pesquise..." />
            <CommandList>
                <CommandEmpty>Sem resultados.</CommandEmpty>
                <CommandGroup heading="Sugestões">
                    <CommandItem>
                        <CalculatorIcon className="mr-2 h-4 w-4" />
                        <span>Calculadora de Propostas</span>
                    </CommandItem>
                </CommandGroup>
                <CommandSeparator />
                <CommandGroup heading="Personalização">
                    <CommandItem>
                        <PersonIcon className="mr-2 h-4 w-4" />
                        <span>Dados do perfil</span>
                        <CommandShortcut>⌘P</CommandShortcut>
                    </CommandItem>
                    <CommandItem>
                        <EnvelopeClosedIcon className="mr-2 h-4 w-4" />
                        <span>E-mail</span>
                        <CommandShortcut>⌘B</CommandShortcut>
                    </CommandItem>
                    <CommandItem>
                        <GearIcon className="mr-2 h-4 w-4" />
                        <span>Configurações</span>
                        <CommandShortcut>⌘S</CommandShortcut>
                    </CommandItem>
                </CommandGroup>
            </CommandList>
        </Command>
    );
}
