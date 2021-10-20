#![allow(clippy::module_name_repetitions)]

use std::{
    cell::RefCell,
    collections::{hash_map::Iter, HashMap},
    ops::AddAssign,
    rc::Rc,
};

type Weight = u64;

/// 字符权重
pub struct CharWeightMap {
    pub inner: HashMap<char, Weight>,
}

/// 计算字符权重
///
/// 计算每个字符出现的次数作为权重
impl CharWeightMap {
    pub fn build(input: &str) -> Self {
        let mut map = HashMap::new();
        for (_, char) in input.char_indices() {
            map.entry(char).or_insert(0).add_assign(1);
        }
        Self { inner: map }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter(&self) -> Iter<char, Weight> {
        self.inner.iter()
    }
}

type RefHuffmanTree = Rc<RefCell<HuffmanTree>>;

/// 霍夫曼树
pub struct HuffmanTree {
    /// 值
    pub value: Option<char>,
    /// 权重
    pub weight: Weight,
    /// 父节点
    pub parent: Option<RefHuffmanTree>,
    /// 左分支
    pub left: Option<RefHuffmanTree>,
    /// 右分支
    pub right: Option<RefHuffmanTree>,
}

impl HuffmanTree {
    pub fn new() -> Self {
        Self {
            value: None,
            weight: 0,
            parent: None,
            left: None,
            right: None,
        }
    }
    pub fn build(char_weight: &CharWeightMap) -> RefHuffmanTree {
        // 原始节点数量
        let n = char_weight.len();
        // 构建完整霍夫曼树总共需要的节点数量
        let total = 2 * n - 1;
        // 初始化所有节点
        let vec = (0..total)
            .map(|_| Rc::new(RefCell::new(Self::new())))
            .collect::<Vec<Rc<RefCell<HuffmanTree>>>>();

        // 字符节点赋值
        char_weight
            .iter()
            .enumerate()
            .into_iter()
            .for_each(|(index, (ch, weight))| {
                vec[index].borrow_mut().value = Some(*ch);
                vec[index].borrow_mut().weight = *weight;
            });

        for index in n..total {
            // 找到 [0, index-1] 中权重最小的节点
            let m1 = Self::find_min(&vec[..index]).unwrap();
            // 标记父节点为 index 上的节点，下次就不会找到这个
            m1.borrow_mut().parent = Some(vec[index].clone());
            // 找到 [0, index-1] 中权重第二小的节点
            let m2 = Self::find_min(&vec[..index]).unwrap();
            // 标记该节点的父节点为 index 上的节点
            m2.borrow_mut().parent = Some(vec[index].clone());

            let w1 = m1.as_ref().borrow().weight;
            let w2 = m2.as_ref().borrow().weight;
            let weight = w1 + w2;

            vec[index].borrow_mut().weight = weight;
            vec[index].borrow_mut().left = Some(m1.clone());
            vec[index].borrow_mut().right = Some(m2.clone());
        }
        // 最后一个节点即是构建好的完整霍夫曼树
        vec.last().unwrap().clone()
    }

    /// 获取最小的值
    fn find_min(tree_slice: &[Rc<RefCell<HuffmanTree>>]) -> Option<Rc<RefCell<HuffmanTree>>> {
        let mut min = Weight::MAX;
        let mut result = None;
        for tree in tree_slice {
            let tree_cell = tree.as_ref();
            if tree_cell.borrow().parent.is_none() && tree_cell.borrow().weight < min {
                min = tree_cell.borrow().weight;
                result = Some(tree.clone());
            }
        }
        result
    }
}
