use image::DynamicImage;
use uuid::Uuid;

#[derive(Default)]
pub struct ImageRepository{
    pub repository: Vec<ImageHistory>,
    pub selected_history_id: usize,
    pub last_history_id: usize
}

impl ImageRepository{
    pub fn set_selected_history_id(&mut self, id: usize){
        self.selected_history_id = id;
    }

    pub fn new_entry(&mut self, image: DynamicImage ) -> usize{
        self.last_history_id += 1;
        
        let mut history = ImageHistory::new();
        history.add_entry(image);
        history.set_id(self.last_history_id);
        
        self.repository.push(history);
        return self.last_history_id;
    }

    pub fn get_history(&mut self) -> Option<&mut ImageHistory>{
        let id = self.selected_history_id;

        self
            .iter_mut()
            .find(|cur| cur.history_id == id)
    }

    pub fn get_history_by_id(&mut self, index: usize) -> Option<&mut ImageHistory>{
        let history = self
            .iter_mut()
            .find(|cur| cur.history_id as usize == index);

        history
    }

    pub fn remove_history(&mut self) -> Option<ImageHistory>{
        let history_index = self
        .iter()
        .position(|x| x.history_id == self.selected_history_id)?;

        let discarded = self.repository.remove(history_index);
        Some(discarded)
    }
    
    pub fn remove_history_by_id(&mut self, id : usize) -> Option<ImageHistory>{
        let history_index = self
            .iter()
            .position(|x| x.history_id == id)?;

        let discarded = self.repository.remove(history_index);
        Some(discarded)
    }

    pub fn iter(&self) -> impl Iterator<Item=&ImageHistory>{
        self.repository.iter()
    }

    pub fn iter_mut(&mut self) -> std::slice::IterMut<ImageHistory> {
        self.repository.iter_mut()
    }

    pub fn change_selected(&mut self, id: usize) -> Option<&ImageHistory> {
        // TODO:Fix this reference shit that makes me unable to edit 'self' after
        //      returning the ImageHistory
        // AFKA:Use only find to determine if the value exists and change the selected id...
        let id_exists = self
            .iter()
            .any(|x| x.history_id == id);

        match id_exists{
            true => {
                self.set_selected_history_id(id);
                self
                    .iter()
                    .find(|x| x.history_id == id)
            },
            false => None,
        }
    }
}

pub struct ImageHistoryNode{
    pub image: DynamicImage,
    pub uuid: Uuid,
    pub psnr: f64
}
impl ImageHistoryNode {
    pub fn new(image: DynamicImage) -> Self{
        let uuid = Uuid::new_v4();
        return Self{image, uuid, psnr: 0f64};
    }

    pub fn new_with_psnr(image: DynamicImage, psnr: f64) -> Self{
        let uuid = Uuid::new_v4();
        return Self{image, uuid, psnr};
    }  
}
pub struct ImageHistory{
    pub entries: Vec<ImageHistoryNode>,
    pub history_id: usize,
    pub selected_image_uuid: Option<Uuid>
}

impl ImageHistory {
    pub fn new() -> Self{
        Self{entries: Vec::new(), history_id: 0, selected_image_uuid: None}
    }

    pub fn undo(&mut self){
        self.entries.pop();
    }

    pub fn set_id(&mut self, id: usize){
        self.history_id = id;
    }

    pub fn add_entry(&mut self, image: DynamicImage) -> Uuid{
        let node = ImageHistoryNode::new(image);
        let uuid = node.uuid;

        self.selected_image_uuid = Some(node.uuid);

        self.entries.push(node);
        uuid
    }

    pub fn change_selected(&mut self, id: &str) -> bool {
        let selected = self
            .entries
            .iter()
            .find(|x| x.uuid.to_string().contains(id));

        match selected {
            Some(entry) => self.selected_image_uuid = Some(entry.uuid),
            None => (),
        }

        selected.is_some()
    }

    pub fn get_entry(&mut self) -> Option<&mut ImageHistoryNode>{
        let uuid = self.selected_image_uuid?;

        let selected = self
            .entries
            .iter_mut()
            .find(|x| x.uuid == uuid);

        selected
    }
}