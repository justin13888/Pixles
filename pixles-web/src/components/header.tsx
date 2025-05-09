import { Avatar, AvatarFallback, AvatarImage } from '@/components/ui/avatar';
import { Button } from '@/components/ui/button';
import {
    DropdownMenu,
    DropdownMenuContent,
    DropdownMenuItem,
    DropdownMenuSeparator,
    DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu';
import { Input } from '@/components/ui/input';

import { APP_NAME } from '@/lib/constant';
import { Link } from '@tanstack/react-router';
import { BellIcon, MountainIcon, UploadIcon } from 'lucide-react';
import { ModeToggle } from './ui/mode-toggle';

// TODO: Implement interactivity
export const Header = () => (
    <header className="w-full bg-background px-4 py-3 shadow-sm dark:bg-muted">
        <div className="container mx-auto flex items-center justify-between">
            <Link to="/" className="flex items-center gap-2">
                <MountainIcon className="size-6 text-primary" />
                <span className="text-lg font-bold text-primary">
                    {APP_NAME}
                </span>
            </Link>
            <div className="flex flex-1 items-center justify-center px-4">
                <Input
                    type="text"
                    placeholder="Search..."
                    className="w-full max-w-md rounded-md border border-input bg-background px-4 py-2 text-sm focus:border-primary focus:outline-none dark:bg-muted"
                />
            </div>
            <div className="flex items-center gap-4">
                <Button variant="outline" size="icon">
                    <UploadIcon className="h-5 w-5" />
                    <span className="sr-only">Upload</span>
                </Button>
                <Button variant="outline" size="icon">
                    <BellIcon className="h-5 w-5" />
                    <span className="sr-only">Notifications</span>
                </Button>
                <ModeToggle />
                <div />
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Avatar className="h-9 w-9">
                            <AvatarImage
                                src="/placeholder-user.jpg"
                                alt="@shadcn"
                            />
                            <AvatarFallback>JP</AvatarFallback>
                        </Avatar>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent>
                        <DropdownMenuItem>My Account</DropdownMenuItem>
                        <DropdownMenuItem>Settings</DropdownMenuItem>
                        <DropdownMenuSeparator />
                        <DropdownMenuItem>Logout</DropdownMenuItem>
                    </DropdownMenuContent>
                </DropdownMenu>
            </div>
        </div>
    </header>
);
