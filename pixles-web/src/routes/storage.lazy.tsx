import {
    Dialog,
    DialogContent,
    DialogDescription,
    DialogHeader,
    DialogTitle,
    DialogTrigger,
} from '@/components/ui/dialog';
import { TabsContent, TabsList, TabsTrigger } from '@/components/ui/tabs';
import { Tabs } from '@/components/ui/tabs';
import { graphql } from '@/gql';
import { type AssetListSearchQueryQuery, AssetType } from '@/gql/graphql';
import { cn } from '@/lib/utils';
import { Link, createLazyFileRoute } from '@tanstack/react-router';
import {
    type SortingState,
    createColumnHelper,
    flexRender,
    getCoreRowModel,
    getSortedRowModel,
    useReactTable,
} from '@tanstack/react-table';
import { filesize } from 'filesize';
import {
    AppWindow,
    ChevronDown,
    ChevronUp,
    FileText,
    FileVideo,
    HelpCircle,
    Image,
    Mail,
    RefreshCw,
    Trash2,
} from 'lucide-react';
import React, { useState } from 'react';
import { useQuery } from 'urql';

export const Route = createLazyFileRoute('/storage')({
    component: () => <Storage />,
});

const StorageQuery = graphql(`
  query StorageQuery {
    user {
      statistics {
        totalPhotos
        totalAlbums
        usedStorage
        usedStoragePhotos
        usedStorageVideos
        usedStorageSidecar
        totalStorage
        usedStorageTrash
        usedStorageSimilarAssets
        usedStorageLargeFiles
      }
    }
  }
`);

const AssetListSearchQuery = graphql(`
  query AssetListSearchQuery($albumIds: [ID!], $filter: AssetFilter, $sort: AssetSort, $sortDirection: SortDirection) {
    asset {
      search(albumIds: $albumIds, filter: $filter, sort: $sort, sortDirection: $sortDirection) {
        id
        type
        fileName
        path
        size
        date
        thumbnail
      }
    }
  }
`);

function formatFileSize(size: number) {
    return filesize(size, {
        base: 2,
        standard: 'jedec',
    });
}

type Suggestion = 'trash-files' | 'similar-assets' | 'large-files';

function Storage() {
    const [{ data, fetching, error }, reexecuteQuery] = useQuery({
        query: StorageQuery,
    });
    const stats = data?.user.statistics;

    const [activeTab, setActiveTab] = useState<Suggestion>('trash-files');

    const [previewAssetId, setPreviewAssetId] = useState<string | undefined>(
        undefined,
    );
    const [previewOpen, setPreviewOpen] = useState(false);

    // Icon mapping for cleanup suggestions
    const getIconForSuggestion = (id: string) => {
        switch (id) {
            case 'trash-files':
                return <Trash2 size={28} className="text-gray-700" />;
            case 'similar-assets':
                return <Image size={28} className="text-gray-700" />;
            case 'large-files':
                return <FileText size={28} className="text-gray-700" />;
            default:
                return <FileText size={28} className="text-gray-700" />;
        }
    };

    return (
        <Dialog open={previewOpen} onOpenChange={setPreviewOpen}>
            <div className="max-w-6xl mx-auto p-6">
                {/* Storage Usage Overview */}
                <div className="mb-10">
                    <h1 className="text-2xl font-medium text-gray-800 mb-4">
                        Manage Storage
                    </h1>
                    {fetching ? (
                        <div>Loading...</div>
                    ) : error ? (
                        <div>Error</div>
                    ) : !stats ? (
                        <div>No data</div>
                    ) : (
                        (() => {
                            const storageData = {
                                totalStorage: stats.totalStorage, // in bytes
                                usedStorage: stats.usedStorage, // in bytes
                                categories: [
                                    {
                                        name: 'Photos',
                                        size: stats.usedStoragePhotos,
                                        color: 'bg-red-500',
                                    },
                                    {
                                        name: 'Videos',
                                        size: stats.usedStorageVideos,
                                        color: 'bg-blue-500',
                                    },
                                    {
                                        name: 'Sidecar',
                                        size: stats.usedStorageSidecar,
                                        color: 'bg-purple-600',
                                    },
                                ],
                                cleanupSuggestions: [
                                    {
                                        id: 'trash-files',
                                        title: 'Files in trash',
                                        size: stats.usedStorageTrash,
                                    },
                                    {
                                        id: 'similar-assets',
                                        title: 'Similar assets',
                                        size: stats.usedStorageSimilarAssets,
                                    },
                                    {
                                        id: 'large-files',
                                        title: 'Large files',
                                        size: stats.usedStorageLargeFiles,
                                    },
                                ],
                            };

                            // Calculate percentage of storage used
                            const totalPercentageUsed =
                                (storageData.usedStorage /
                                    storageData.totalStorage) *
                                100;

                            return (
                                <>
                                    <h2 className="text-xl font-medium text-gray-800 mb-4">
                                        {totalPercentageUsed.toFixed(0)}% of
                                        storage used (
                                        {formatFileSize(
                                            storageData.usedStorage,
                                        )}{' '}
                                        of{' '}
                                        {formatFileSize(
                                            storageData.totalStorage,
                                        )}
                                        )
                                    </h2>
                                    <p className="text-gray-600 mb-6">
                                        Make room for your photos, files, and
                                        more by cleaning up space
                                    </p>

                                    {/* Progress Bar */}
                                    <div className="h-2 w-full rounded-full overflow-hidden flex">
                                        {storageData.categories.map(
                                            (category) => (
                                                <div
                                                    key={category.name}
                                                    className={`h-full ${category.color}`}
                                                    style={{
                                                        width: `${(category.size / storageData.totalStorage) * 100}%`,
                                                    }}
                                                />
                                            ),
                                        )}
                                        <div
                                            className="h-full bg-gray-200"
                                            style={{
                                                width: `${100 - totalPercentageUsed}%`,
                                            }}
                                        />
                                    </div>

                                    {/* Legend */}
                                    <div className="flex flex-wrap mt-4 justify-between">
                                        <div className="flex flex-wrap">
                                            {storageData.categories.map(
                                                (category) => (
                                                    <div
                                                        key={category.name}
                                                        className="flex items-center mr-6 mb-2"
                                                    >
                                                        <div
                                                            className={cn(
                                                                'w-3 h-3 rounded-full',
                                                                category.color,
                                                                'mr-2',
                                                            )}
                                                        />
                                                        <span className="text-sm text-gray-700">
                                                            {category.name} (
                                                            {formatFileSize(
                                                                category.size,
                                                            )}
                                                            )
                                                        </span>
                                                    </div>
                                                ),
                                            )}
                                        </div>
                                        <div className="flex items-center mb-2">
                                            <div
                                                className={cn(
                                                    'w-3 h-3 rounded-full',
                                                    'bg-gray-200',
                                                    'mr-2',
                                                )}
                                            />
                                            <span className="text-sm text-gray-700">
                                                Free Space (
                                                {formatFileSize(
                                                    storageData.totalStorage -
                                                        storageData.usedStorage,
                                                )}
                                                )
                                            </span>
                                        </div>
                                    </div>

                                    {/* Clean Up Suggestions */}
                                    <div>
                                        <h2 className="text-xl font-medium text-gray-800 mb-6">
                                            Clean up suggested items
                                        </h2>
                                        <Tabs
                                            value={activeTab}
                                            onValueChange={(value) =>
                                                setActiveTab(
                                                    value as Suggestion,
                                                )
                                            }
                                            className="w-full"
                                        >
                                            <TabsList className="mb-4">
                                                {storageData.cleanupSuggestions.map(
                                                    (suggestion) => (
                                                        <TabsTrigger
                                                            key={suggestion.id}
                                                            value={
                                                                suggestion.id
                                                            }
                                                        >
                                                            <div className="flex items-center gap-2">
                                                                {getIconForSuggestion(
                                                                    suggestion.id,
                                                                )}
                                                                {
                                                                    suggestion.title
                                                                }
                                                            </div>
                                                        </TabsTrigger>
                                                    ),
                                                )}
                                            </TabsList>
                                            {storageData.cleanupSuggestions.map(
                                                (suggestion) => {
                                                    const [
                                                        {
                                                            data: assetData,
                                                            fetching:
                                                                assetFetching,
                                                            error: assetError,
                                                        },
                                                        reexecuteAssetQuery,
                                                    ] = useQuery({
                                                        query: AssetListSearchQuery,
                                                        pause:
                                                            activeTab !==
                                                            suggestion.id,
                                                    });
                                                    return (
                                                        <TabsContent
                                                            key={suggestion.id}
                                                            value={
                                                                suggestion.id
                                                            }
                                                        >
                                                            <div className="flex flex-col">
                                                                {assetFetching ? (
                                                                    <div>
                                                                        Loading...
                                                                    </div>
                                                                ) : assetError ? (
                                                                    <div>
                                                                        Error:{' '}
                                                                        {
                                                                            assetError.message
                                                                        }
                                                                    </div>
                                                                ) : assetData ? (
                                                                    <AssetList
                                                                        assets={
                                                                            assetData
                                                                                .asset
                                                                                .search
                                                                        }
                                                                    />
                                                                ) : (
                                                                    <div>
                                                                        No data
                                                                    </div>
                                                                )}
                                                                <div className="flex justify-end mb-4">
                                                                    <button
                                                                        type="button"
                                                                        onClick={() =>
                                                                            reexecuteAssetQuery(
                                                                                {
                                                                                    requestPolicy:
                                                                                        'network-only',
                                                                                },
                                                                            )
                                                                        }
                                                                        className="flex items-center gap-2 px-3 py-2 text-sm font-medium text-gray-700 bg-white border border-gray-300 rounded-md hover:bg-gray-50 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-blue-500"
                                                                    >
                                                                        <RefreshCw className="h-4 w-4" />
                                                                        Refresh
                                                                    </button>
                                                                </div>
                                                            </div>
                                                        </TabsContent>
                                                    );
                                                },
                                            )}
                                        </Tabs>
                                    </div>
                                </>
                            );
                        })()
                    )}
                </div>
            </div>
            <DialogContent>
                <DialogHeader>
                    <DialogTitle>Preview</DialogTitle>
                    <DialogDescription>
                        {previewAssetId}
                        {/* TODO: Add preview */}
                    </DialogDescription>
                </DialogHeader>
            </DialogContent>
        </Dialog>
    );
}

// Asset type definition
type Asset = {
    id: string;
    type: AssetType;
    fileName: string;
    path: string;
    size: number;
    date: Date;
    thumbnail?: string;
};

// AssetList component
function AssetList({
    assets,
}: { assets: AssetListSearchQueryQuery['asset']['search'] }) {
    if (assets.length === 0) {
        return <div>Empty asset list</div>;
    }

    // Column helper for type safety
    const columnHelper = createColumnHelper<Asset>();

    // Define columns
    const columns = [
        columnHelper.accessor('thumbnail', {
            header: '',
            cell: (info) => (
                <div className="flex items-center justify-center w-12 h-12">
                    {info.getValue() ? (
                        <div className="w-10 h-10 bg-blue-100 rounded overflow-hidden">
                            <img
                                src={info.getValue()}
                                alt={`Thumbnail for ${info.row.original.fileName}`}
                                className="w-full h-full object-cover"
                            />
                        </div>
                    ) : (
                        <div className="w-10 h-10 bg-gray-100 rounded flex items-center justify-center">
                            {info.row.original.type === AssetType.Photo ? (
                                <Image
                                    className="size-6 text-blue-500"
                                    aria-label={`Image file: ${info.row.original.fileName}`}
                                />
                            ) : info.row.original.type === AssetType.Video ? (
                                <FileVideo
                                    className="size-6 text-red-500"
                                    aria-label={`Video file: ${info.row.original.fileName}`}
                                />
                            ) : info.row.original.type === AssetType.Sidecar ? (
                                <FileText
                                    className="size-6 text-purple-500"
                                    aria-label={`Sidecar file: ${info.row.original.fileName}`}
                                />
                            ) : (
                                // This exhaustive check ensures all AssetType values are handled
                                (() => {
                                    // @ts-expect-error: This is an exhaustive check
                                    const _exhaustiveCheck: never =
                                        info.row.original.type;
                                    return null;
                                })()
                            )}
                        </div>
                    )}
                </div>
            ),
            enableSorting: false,
        }),
        columnHelper.accessor('fileName', {
            header: 'File Name',
            cell: (info) => (
                <div className="font-medium">{info.getValue()}</div>
            ),
        }),
        columnHelper.accessor('path', {
            header: 'Path',
            cell: (info) => (
                <div className="text-gray-500 text-sm">{info.getValue()}</div>
            ),
        }),
        columnHelper.accessor('size', {
            header: () => <div className="text-right">Size</div>,
            cell: (info) => (
                <div className="text-right">
                    {formatFileSize(info.getValue())}
                </div>
            ),
            sortingFn: 'basic',
        }),
        columnHelper.accessor('date', {
            header: () => <div className="text-right">Date</div>,
            cell: (info) => (
                <div className="text-right">
                    {info.getValue().toLocaleDateString()}
                </div>
            ),
            sortingFn: 'datetime',
        }),
    ];

    // Fix the sorting state type
    const [sorting, setSorting] = React.useState<SortingState>([]);

    const table = useReactTable({
        data: assets,
        columns,
        state: {
            sorting,
        },
        onSortingChange: setSorting,
        getCoreRowModel: getCoreRowModel(),
        getSortedRowModel: getSortedRowModel(),
    });

    return (
        <>
            <div className="rounded-md border">
                <table className="w-full">
                    <thead>
                        {table.getHeaderGroups().map((headerGroup) => (
                            <tr
                                key={headerGroup.id}
                                className="border-b bg-gray-50"
                            >
                                {headerGroup.headers.map((header) => (
                                    <th
                                        key={header.id}
                                        className="px-4 py-3 text-left text-sm font-medium text-gray-700"
                                        onClick={header.column.getToggleSortingHandler()}
                                        onKeyDown={(e) => {
                                            if (
                                                e.key === 'Enter' ||
                                                e.key === ' '
                                            ) {
                                                header.column.getToggleSortingHandler();
                                            }
                                        }}
                                        style={{
                                            cursor: header.column.getCanSort()
                                                ? 'pointer'
                                                : 'default',
                                        }}
                                        tabIndex={
                                            header.column.getCanSort()
                                                ? 0
                                                : undefined
                                        }
                                        role={
                                            header.column.getCanSort()
                                                ? 'button'
                                                : undefined
                                        }
                                    >
                                        <div className="flex items-center">
                                            {flexRender(
                                                header.column.columnDef.header,
                                                header.getContext(),
                                            )}
                                            {header.column.getCanSort() && (
                                                <span className="ml-1">
                                                    {header.column.getIsSorted() ===
                                                    'asc'
                                                        ? '🔼'
                                                        : header.column.getIsSorted() ===
                                                            'desc'
                                                          ? '🔽'
                                                          : '⏺️'}
                                                </span>
                                            )}
                                        </div>
                                    </th>
                                ))}
                            </tr>
                        ))}
                    </thead>
                    <tbody>
                        {table.getRowModel().rows.map((row) => (
                            <tr
                                key={row.id}
                                className="border-b hover:bg-gray-50"
                            >
                                {row.getVisibleCells().map((cell) => (
                                    <td
                                        key={cell.id}
                                        className="px-4 py-3 text-sm"
                                        // onKeyDown={() => {
                                        //   setPreviewAssetId(row.id);
                                        //   setPreviewOpen(true);
                                        // }} // TODO: Fix this
                                    >
                                        {flexRender(
                                            cell.column.columnDef.cell,
                                            cell.getContext(),
                                        )}
                                    </td>
                                ))}
                            </tr>
                        ))}
                    </tbody>
                </table>
            </div>
        </>
    );
}
