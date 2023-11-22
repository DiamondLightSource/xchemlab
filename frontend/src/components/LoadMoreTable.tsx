import { Box, BoxProps, Heading, Table, Tbody, Td, Th, Thead, Tr, Button, HStack } from "@chakra-ui/react";
import React from "react";
import { useCallback } from "react";

export interface TableProps extends Omit<BoxProps, "onClick"> {
  /** Table data */
  data: Record<string, any>[] | null;
  /** Table headers and mapping to record keys */
  headers: { key: string; label: string }[];
  /** Callback when row is clicked */
  onRowClick?: (item: Record<string, any>, index: number) => void;
  /** Label to be used when displaying "no data available" message */
  label?: string;
  /** Styling variant to use for the rows */
  rowVariant?: string;
  /** feed click behaviour into Load More button */
  onButtonClick?: React.MouseEventHandler<HTMLButtonElement>
}

const TableView = ({
  data,
  headers,
  onRowClick,
  label = "data",
  rowVariant = "diamondStriped",
  onButtonClick,
  ...props
}: TableProps) => {
  const handleClick = useCallback(
    (row: React.MouseEvent<HTMLTableRowElement>) => {
      if (onRowClick && data) {
        const target = (row.target as HTMLTableCellElement).dataset.id;
        if (target) {
          const intTarget = parseInt(target);
          onRowClick(data[intTarget], intTarget);
        }
      }
    },
    [data, onRowClick],
  );

  return (
    <>
    <Box overflowY='scroll' {...props}>
      {data === null || data.length === 0 ? (
        <Heading py={10} w='100%' variant='notFound'>
          No {label.toLowerCase()} found
        </Heading>
      ) : (
        <Table size='sm' variant={rowVariant}>
          <Thead>
            <Tr>
              {headers.map((header) => (
                <Th key={header.label}>{header.label}</Th>
              ))}
            </Tr>
          </Thead>
          <Tbody cursor='pointer'>
            {data.map((item, i) => (
              <Tr h='2vh' key={i} onClick={handleClick}>
                {headers.map((header) => (
                  <Td data-id={i} key={header.key}>
                    {item[header.key]}
                  </Td>
                ))}
              </Tr>
            ))}
          </Tbody>
        </Table>
      )}
    </Box>
    <HStack justify='center' width='100%'>
      <Button colorScheme='teal' variant='outline' onClick={onButtonClick}>
        Load More
      </Button>
    </HStack>
    </>
  );
};

export { TableView as LoadMoreTable };