"use client";
import { useRef, useState } from "react";
import ky from "ky";
import { ItemDataType } from "@/types/userModel";
import { SearchItemResults } from "@/app";

const SearchItem = () => {
  const searchItemElement = useRef<HTMLInputElement>(null);
  const [searchItemResult, setSearchItemResult] = useState<ItemDataType[]>([]);
  const searchItemHandler = async (e: React.FormEvent) => {
    e.preventDefault();
    const inputValue: string | undefined = searchItemElement.current?.value;
    if (!inputValue) return;
    //複数回繰り返しスペースがある場合の修正をする
    const inputReplaceBlankValue = inputValue.replace(/\s+/g, "+");
    const url = `http://localhost:5000/api/item/search?keywords=${inputReplaceBlankValue}`;
    try {
      const result: ItemDataType[] = await ky.get(url).json();
      setSearchItemResult(result);
      console.log(`URL: ${url}`);
      console.log(searchItemResult);
    } catch (error) {
      console.error(error);
    }
  };
  return (
    <>
      <form>
        <input
          type="text"
          placeholder="Search"
          ref={searchItemElement}
          autoFocus
        />
        <button onClick={searchItemHandler}>Search</button>
      </form>
      <SearchItemResults data={searchItemResult} />
    </>
  );
};

export default SearchItem;
