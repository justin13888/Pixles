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
import { UploadDialog } from './upload-dialog';

// TODO: Implement interactivity
export const Header = () => (
    <header className="w-full bg-background px-4 py-3 shadow-sm dark:bg-muted border-b">
        <div className="flex items-center justify-between w-full">
            <Link to="/" className="flex items-center gap-2">
                <MountainIcon className="size-6 text-primary" />
                <span className="text-lg font-bold text-primary">
                    {APP_NAME}
                </span>
            </Link>
            <div className="flex flex-1 items-center justify-center px-4">
                <Input
                    type="text"
                    placeholder="Search your photos..."
                    className="w-full max-w-md rounded-md bg-muted px-4 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-primary"
                />
            </div>
            <div className="flex items-center gap-2">
                <UploadDialog>
                    <Button variant="ghost" size="icon">
                        <UploadIcon className="h-5 w-5" />
                        <span className="sr-only">Upload</span>
                    </Button>
                </UploadDialog>
                <Button variant="ghost" size="icon">
                    <BellIcon className="h-5 w-5" />
                    <span className="sr-only">Notifications</span>
                </Button>
                <ModeToggle />
                <div className="w-2" />
                <DropdownMenu>
                    <DropdownMenuTrigger asChild>
                        <Avatar className="h-8 w-8 cursor-pointer">
                            <AvatarImage
                                src="/placeholder-user.jpg"
                                alt="@shadcn"
                            />
                            <AvatarFallback>JP</AvatarFallback>
                        </Avatar>
                    </DropdownMenuTrigger>
                    <DropdownMenuContent align="end">
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
