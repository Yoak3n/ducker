import React, { useState, useCallback, useMemo, useEffect } from 'react';
import { useParams,useNavigate } from 'react-router-dom';
import { z } from "zod";
import { zodResolver } from "@hookform/resolvers/zod"
import { useForm } from "react-hook-form"

import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Tooltip, TooltipContent, TooltipTrigger, } from "@/components/ui/tooltip"
import { Input } from '@/components/ui/input';
import { Card, CardAction, CardContent, CardHeader, CardTitle, } from "@/components/ui/card"
import { Select, SelectContent, SelectGroup, SelectItem, SelectLabel, SelectTrigger, SelectValue, } from "@/components/ui/select"
import { Button } from '@/components/ui/button';
import { Textarea } from '@/components/ui/textarea';
import { Badge } from '@/components/ui/badge';
import { Skeleton } from "@/components/ui/skeleton"
import { toast } from "sonner"

import { useActionStore } from '@/store';
import { open_file_select_dialog, simplifyPath } from '@/utils';
import type { Action } from '@/types';

const formSchema = z.object({
    name: z.string().min(2, {
        message: "action name must be at least 2 characters.",
    }),
    type: z.enum(["open_file", "open_dir", "exec_command", "open_url"]),
    command: z.string(),
    args: z.string().optional(),
    desc: z.string(),
    wait: z.coerce.number(),
})

const ActionModify: React.FC = () => {
    const [isFileType, setIsFileType] = useState<boolean>(true);
    const [isUrlType, setIsUrlType] = useState<boolean>(false);
    const [isCommandType, setIsCommandType] = useState<boolean>(false);

    const actionStore = useActionStore();
    const navigate = useNavigate();
    const params = useParams()
    const id = params.id;

    const form = useForm<z.infer<typeof formSchema>>({
        resolver: zodResolver(formSchema),
        defaultValues: {
            name: "",
            type: "open_file",
            command: "",
            desc: "",
            wait: 0,
        },
    })

    const handleTypeChange = useCallback((type: string) => {
        setIsFileType(type === "open_file" || type === "open_dir");
        setIsUrlType(type === "open_url");
        setIsCommandType(type === "exec_command");
    }, []);

    const currentAction = useMemo(() => {
        const action = id !== "add" ? actionStore.actions.find(action => action.id === id) : null;
        return action;
    }, [id, actionStore.actions]);
    const isAddAction = useMemo(() => {
        return id === "add";
    }, [id]);

    useEffect(() => {
        if (isAddAction) {
            const defaultValues = {
                name: "",
                type: "open_file" as const,
                command: "",
                args: "",
                desc: "",
                wait: 0,
            };
            form.reset(defaultValues);
            handleTypeChange("open_file");
        } else if (currentAction) {
            const formData = {
                name: currentAction.name,
                type: currentAction.type,
                command: currentAction.command || "",
                args: currentAction.args?.join(",") || "",
                desc: currentAction.desc,
                wait: currentAction.wait,
            };
            form.reset(formData);
            handleTypeChange(currentAction.type);
        }
    }, [currentAction, form, handleTypeChange, id]);


    const actionTypeOptions = [
        { value: "open_file", label: "Open File" },
        { value: "open_dir", label: "Open Directory" },
        { value: "exec_command", label: "Execute Command" },
        { value: "open_url", label: "Open URL" },
    ];
    const onSubmit = (values: z.infer<typeof formSchema>) => {
        console.log('onSubmit', values);
        const args = values.args?.split(",")
        const actionData = { ...values, args, type: values.type as Action['type'] }
        if (isAddAction) {
            actionStore.createAction(actionData).then(() => {
                navigate("/action");
                toast.success("Action created successfully");
            }).catch(e => {
                console.error(e)
            })
        } else {
            actionStore.updateAction(id!, actionData).then(() => {
                navigate("/action");
                toast.success("Action updated successfully");
            }).catch(e => {
                console.error(e)
            })
        }
    }

    const handleSelectFile = useCallback(async () => {
        const isFile = form.getValues("type") === "open_file";
        const path = await open_file_select_dialog(isFile);
        form.setValue("command", path!);
    }, [form]);


    return (
        <>
        <Card className="action-modify" >
            <CardHeader>
                <CardTitle>{isAddAction ? "Add Action" : "Modify Action"}</CardTitle>
                <CardAction className='cursor-pointer' onClick={() => navigate("/action")}>
                    <span className="material-symbols-outlined">
                        arrow_back
                    </span>
                </CardAction>
            </CardHeader>
            <CardContent>
                <Form {...form} >
                    <form
                        onSubmit={form.handleSubmit(onSubmit)}
                    >
                        <div className='grid gap-4 grid-cols-2 mb-1'>
                            <FormField
                                control={form.control}
                                name="name"
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Action Name</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter action name"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />

                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                            <FormField
                                name="type"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Type</FormLabel>
                                        <Select value={field.value || "open_file"} onValueChange={(e) => {
                                            if (e && e != '') {
                                                field.onChange(e)
                                                handleTypeChange(e)
                                            }

                                        }}>
                                            <FormControl>
                                                <SelectTrigger className="w-5/6">
                                                    <SelectValue placeholder="Select action type" />
                                                </SelectTrigger>
                                            </FormControl>
                                            <SelectContent>
                                                <SelectGroup>
                                                    <SelectLabel>Basic Type</SelectLabel>
                                                    {actionTypeOptions.map((option) => (
                                                        <SelectItem key={option.value} value={option.value}>
                                                            {option.label}
                                                        </SelectItem>
                                                    ))}
                                                </SelectGroup>
                                            </SelectContent>
                                        </Select>

                                    </FormItem>
                                )}
                            />
                        </div>
                        <FormField
                            control={form.control}
                            name="desc"
                            render={({ field }) => (
                                <FormItem className='w-5/6 mb-2'>
                                    <FormLabel className="block text-sm font-medium text-gray-700">Desc</FormLabel>
                                    <FormControl>
                                        <Textarea
                                            {...field}
                                            placeholder="Enter action desc"
                                            className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500 resize-none"
                                        />
                                    </FormControl>
                                    <p className="text-muted-foreground text-sm">
                                        Describe the action.
                                    </p>
                                </FormItem>
                            )}
                        />
                        <div className='grid gap-4 grid-cols-2 '>
                            <FormField
                                name="command"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700 ">{isUrlType ? "Url" : isCommandType ? "Command" : "Path"}</FormLabel>
                                        <FormControl>
                                            {!isFileType ? <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter command"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            /> : field.value != '' ?
                                                <Tooltip>
                                                    <TooltipTrigger asChild>
                                                        <Badge className='justify-start'>
                                                            {simplifyPath(field.value)}
                                                        </Badge>
                                                    </TooltipTrigger>
                                                    <TooltipContent >
                                                        {field.value}
                                                    </TooltipContent>
                                                </Tooltip> : <Skeleton className="h-5 w-[100px]" />
                                            }
                                        </FormControl>
                                    </FormItem>
                                )}
                            />
                            {isCommandType ? <FormField
                                name="args"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Args</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="text"
                                                placeholder="Enter arguments (comma separated)"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />
                                        </FormControl>
                                    </FormItem>
                                )}
                            /> : isFileType ? <Button
                                variant="ghost"
                                className='w-5/6 h-full cursor-pointer'
                                onClick={(e) => { e.preventDefault(); handleSelectFile() }}>
                                <span className="material-symbols-outlined">upload_file</span>
                            </Button> : null
                            }
                        </div>
                        <div className="grid gap-4 grid-cols-2">
                            <FormField
                                name="wait"
                                control={form.control}
                                render={({ field }) => (
                                    <FormItem className='w-5/6 mb-2'>
                                        <FormLabel className="block text-sm font-medium text-gray-700">Wait</FormLabel>
                                        <FormControl>
                                            <Input
                                                {...field}
                                                type="number"
                                                placeholder="Enter wait time"
                                                className="mt-1 block w-full border-gray-300 rounded-md shadow-sm focus:ring-blue-500 focus:border-blue-500"
                                            />
                                        </FormControl>
                                        <FormMessage />
                                    </FormItem>
                                )}
                            />
                        </div>
                        <div className="flex mt-5">
                            <Button className="col-span-2 w-1/2 mx-auto cursor-pointer">Submit</Button>
                        </div>
                    </form>
                </Form>
            </CardContent>
        </Card>
        </>
    );
};

export default ActionModify;

