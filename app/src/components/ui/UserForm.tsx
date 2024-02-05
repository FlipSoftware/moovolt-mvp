import { zodResolver } from "@hookform/resolvers/zod";
import { useForm } from "react-hook-form";
import { z } from "zod";

import { Button } from "@/components/ui/button";
import {
    Form,
    FormControl,
    FormDescription,
    FormField,
    FormItem,
    FormMessage,
} from "@/components/ui/form";
import { Input } from "@/components/ui/input";

import {
    Select,
    SelectContent,
    SelectGroup,
    SelectItem,
    SelectTrigger,
    SelectValue,
} from "@/components/ui/select";

import {
    Card,
    CardContent,
    CardDescription,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { useState } from "react";

import {
    Drawer,
    DrawerClose,
    DrawerContent,
    DrawerDescription,
    DrawerFooter,
    DrawerHeader,
    DrawerTitle,
} from "./drawer";
import { produtos } from "@/protudos";

const UFsBrasil: string[] = [
    "AC",
    "AL",
    "AP",
    "AM",
    "BA",
    "CE",
    "DF",
    "ES",
    "GO",
    "MA",
    "MT",
    "MS",
    "MG",
    "PA",
    "PB",
    "PR",
    "PE",
    "PI",
    "RJ",
    "RN",
    "RS",
    "RO",
    "RR",
    "SC",
    "SP",
    "SE",
    "TO",
];

const formSchema = z.object({
    nome: z.string().min(2, {
        message: "Nome precisa ter pelo menos 2 characteres.",
    }),
    apelido: z.string().optional(),
    CPF: z.string().refine((value) => value.length == 11, {
        message: "CPF inválido. Confira seu documento e digite novamente.",
    }),
    UF: z.string(),
    produto: z.string().optional(),
    cel: z.string().optional(),
    email: z.string().email(),
    credito: z.number(),
});

export function UserForm() {
    const [profile, setProfile] = useState<z.infer<typeof formSchema>>();
    const [openDialog, setOpenDialog] = useState(false);
    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            nome: "",
            apelido: "",
            CPF: "",
            UF: "",
            produto: "",
            cel: "",
            email: "",
            credito: 0,
        },
    });

    function onSubmit(values: z.infer<typeof formSchema>) {
        // ✅ This will be type-safe and validated.
        setProfile(values);
        setOpenDialog(!openDialog);
    }

    return (
        <>
            <Card className="max-w-[800px] flex-justify-self-center">
                <CardHeader>
                    <CardTitle>Calculadora de Propostas</CardTitle>
                    <CardDescription>
                        Calcule facilmente quanto você precisa investir.
                    </CardDescription>
                </CardHeader>
                <CardContent>
                    <Form {...form}>
                        <form
                            onSubmit={form.handleSubmit(onSubmit)}
                            className="grid gap-4"
                        >
                            <span className="text-red-6">
                                campos com * são obrigatórios
                            </span>
                            <FormField
                                control={form.control}
                                name="nome"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormControl>
                                            <Input
                                                placeholder="Nome"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormDescription>
                                            Preencha com seu nome e sobrenome.
                                            <span className="text-red-6">
                                                *
                                            </span>
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="apelido"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormControl>
                                            <Input
                                                placeholder="Apelido (opcional)"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormDescription>
                                            Digite um apelido que gostaria de
                                            usar para sua conta.
                                            <span className="text-red-6">
                                                *
                                            </span>
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="CPF"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormControl>
                                            <Input
                                                placeholder="CPF"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormDescription>
                                            Cadastro de Pessoa Física.
                                            <span className="text-red-6">
                                                *
                                            </span>
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="UF"
                                render={() => (
                                    <Select>
                                        <SelectTrigger>
                                            <SelectValue placeholder="UF" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectGroup>
                                                {UFsBrasil.map((uf) => (
                                                    <SelectItem
                                                        key={uf}
                                                        value={uf}
                                                    >
                                                        {uf}
                                                    </SelectItem>
                                                ))}
                                            </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="produto"
                                render={() => (
                                    <Select>
                                        <SelectTrigger>
                                            <SelectValue placeholder="Produto" />
                                        </SelectTrigger>
                                        <SelectContent>
                                            <SelectGroup>
                                                {produtos.map((produto) => (
                                                    <SelectItem
                                                        key={produto}
                                                        value={produto}
                                                    >
                                                        ID do Produto: {produto}
                                                    </SelectItem>
                                                ))}
                                            </SelectGroup>
                                        </SelectContent>
                                    </Select>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="cel"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormControl>
                                            <Input
                                                placeholder="Número de celular"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="email"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormControl>
                                            <Input
                                                placeholder="Endereço de e-mail."
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormDescription>
                                            Ex: meunome@gmail.com
                                            <span className="text-red-6">
                                                *
                                            </span>
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                control={form.control}
                                name="credito"
                                render={({ field }) => (
                                    <FormItem>
                                        <FormControl>
                                            <Input
                                                placeholder="Crédito"
                                                {...field}
                                            />
                                        </FormControl>
                                        <FormDescription>
                                            <span className="text-red-6">
                                                *
                                            </span>
                                        </FormDescription>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <Button
                                variant="outline"
                                type="submit"
                                onClick={() => openDialog}
                            >
                                Continuar
                            </Button>
                            <Drawer open={openDialog}>
                                <DrawerContent>
                                    <DrawerHeader>
                                        <DrawerTitle>
                                            Revise seus dados
                                        </DrawerTitle>
                                        <h1 className="">
                                            Nome: {profile?.nome}
                                        </h1>
                                        <h2 className="">
                                            Apelido:{" "}
                                            {profile?.apelido
                                                ? profile.apelido
                                                : "nenhum"}
                                        </h2>
                                        <h2 className="">
                                            CPF: {profile?.CPF}
                                        </h2>
                                        <h2 className="">UF: {profile?.UF}</h2>
                                        <h3 className="">
                                            Cel: {profile?.cel}
                                        </h3>
                                        <h3 className="">
                                            Email: {profile?.email}
                                        </h3>
                                        Crédito: {profile?.credito}
                                        <DrawerDescription>
                                            Seus dados serão analisados após o
                                            envio.
                                        </DrawerDescription>
                                    </DrawerHeader>
                                    <DrawerFooter>
                                        <DrawerClose>
                                            <Button variant="outline">
                                                Cancelar
                                            </Button>
                                        </DrawerClose>
                                    </DrawerFooter>
                                </DrawerContent>
                            </Drawer>
                        </form>
                    </Form>
                </CardContent>
            </Card>
        </>
    );
}
