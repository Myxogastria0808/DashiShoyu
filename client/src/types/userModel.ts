export type ItemDataType = {
  id: number;
  visible_id: string;
  parent_id: number;
  parent_visible_id: string;
  grand_parent_id: number;
  grand_parent_visible_id: string;
  name: string;
  product_number: string;
  photo_url: string;
  record: string;
  color: string;
  description: string;
  year_purchased: number | null;
  connector: string[];
  created_at: string;
  updated_at: string;
};
